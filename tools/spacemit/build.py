#!/usr/bin/python3
# -*- coding: utf-8 -*-
import os
import sys
import time
import argparse
import binascii
import struct
# import yaml
import io
import re
import tempfile
import subprocess
import json
import ctypes as ct

# private module
import common_decorator


class ImageBinary(object):
    """First Stage Boot Loader
    """
    def __init__(self, log):
        super().__init__()
        self.LOG = log
        self.input_path = '.'
        self.arch = 2
        self.info_key_str = 'info'
        self.image_key = 'image'
        self.image_module_key = 'module'
        self.image_data_key = 'data'
        self.al_key_str = 'algorithm'

        # info during build operation: type: info list
        self.build_info_dict = {}
        self.temp_file_list = []
        self.key_file_list = []

        self.time_formt = {
            'year'      : '%Y',
            'month'     : '%m',
            'day'       : '%d',
            'hour'      : '%H',
            'minute'    : '%M',
            'second'    : '%S',
        }

        # operation keyword in structure
        self.key_method_dict = {
            "sizeof"    : self.__sizeof,
            "sum32"     : self.__sum32,
            "crc32"     : self.__crc32,
        }
        self.key_method_p = re.compile(r"""
        ^                   # beginning of string
        \s*                 # 0 or more whitespace character or new line
        (sizeof|sum32|crc32)    # method key
        \s*                 # 0 or more whitespace character or new line
        \(
        (.+)                # string inside the parenthesis
        \)
        \s*                 # 0 or more whitespace character or new line
        $                   # end of string
        """, re.X + re.M + re.I)

        self.preprocess_data_type_tuple = ("pubkey", "file")
        self.data_type_op_dict = {
            "structure" : self.__build_structure_data,
            "pubkey"    : self.__build_public_key_data,
            "signature" : self.__build_signature_data,
            "hash"      : self.__build_hash_data,
            "file"      : self.__build_file_data,
        }

        # openssl command pattern for RSA
        self.rsa_cmd_para_dict = {
            "private"   : ("openssl genrsa -out {0} {1}", 2),
            "prvparse"  : ("openssl rsa -in {0} -text -noout -out {1}", 2),
            "public"    : ("openssl rsa -in {0} -pubout -out {1}", 2),
            "pubparse"  : ("openssl rsa -in {0} -pubin -noout -text -out {1}", 2),
            "sign"      : ("openssl dgst -{3} -sign {0} -out {2} {1}", 4),
        }
        # openssl command pattern for ECC
        self.ecc_cmd_para_dict = {
            "private"   : ("openssl ecparam -genkey -name {1} -noout -out {0}", 2),
            "prvparse"  : ("openssl ec -in {0} -text -noout -out {1}", 2),
            "public"    : ("openssl ec -in {0} -pubout -out {1}", 2),
            "pubparse"  : ("openssl ec -in {0} -pubin -text -noout -out {1}", 2),
            "sign"      : ("openssl dgst -{3} -sign {0} -out {2} {1}", 4),
        }

        # hash info: type, hash data size, hash command key
        self.hash_type_tuple = ("sha256", "sha384", "sha512")
        # openssl command pattern for HASH
        self.hash_cmd_tuple = ("openssl dgst -{2} -binary -out {1} {0}", 3)

        self.encrypt_method_dict = {
            "RSA"       : (self._openssl_operation, self.rsa_cmd_para_dict, 'Modulus'),
            "ECC"       : (self._openssl_operation, self.ecc_cmd_para_dict, 'pub'),
        }

    def _run_shell_cmd(self, cmd_str):
        cmd_list = cmd_str.split()
        ret = subprocess.run(cmd_list)
        if ret.returncode:
            self.LOG.error('Return code for "%s" is %d' %(cmd_str, ret.returncode))
        return ret.returncode

    def _magic2int(self, input_str):
        """Convert magic string to number, lsb in the most left.
        """
        return int(binascii.hexlify(input_str[::-1].encode('utf-8')), 16)

    def _atoi(self, input_str):
        """convert digital string to integer.
        """
        try:
            return int(input_str, 0)
        except:
            self.LOG.debug(f"{input_str} is NOT a number")
            return None

    def _itob(self, integer):
        """convert integer to binary string(little endian).
        """
        hex_str = f"{integer:x}"
        if 0 != len(hex_str) % 2:
            hex_str = '0' + hex_str
        return binascii.unhexlify(hex_str)[::-1]

    def _str_add(self, input_str):
        info_list = [i for i in map(lambda x:x.strip(), input_str.split('+'))]
        data_list = []
        for info_str in info_list:
            if self._atoi(info_str):
                data_list.append(self._atoi(info_str))
            elif info_str.endswith(')') and '(' in info_str:
                pass
        return sum(data_list)

    def _align_binary_data(self, binary_data, align_size):
        """Align binary data with specified size.
        """
        size = len(binary_data)
        if align_size > 1:
            size = (size + align_size - 1) // align_size * align_size
        if size and size > len(binary_data):
            binary_data += b"\x00" * (size - len(binary_data))
        elif size and size < len(binary_data):
            self.LOG.error(f"Binary data {size} is sufficient")
            binary_data = binary_data[:size]
        return binary_data

    def _struct_member_convert(self, value_str, size):
        """Convert structure member to little endian binary string
        """
        value = self._atoi(value_str)
        if value is not None:
            binary_str = self._itob(value)
        else:
            # convert string to binary string
            binary_str = value_str.encode('utf-8')
        return self._align_binary_data(binary_str, size)

    def _time_convert(self, input_str):
        format_list = input_str.split('$')
        time_list = []
        for format_str in format_list:
            if format_str in self.time_formt:
                time_list.append(time.strftime(self.time_formt[format_str]))
        return time_list

    def _sum32(self, data):
        """Calculate check sum in uint32 data.
        """
        if isinstance(data, str):
            data = data.encode('utf-8')
        elif not isinstance(data, bytes):
            data = bytearray(data)

        data_words = len(data) // 4
        if data_words * 4 != len(data):
            data = data[:data_words * 4]
        return sum(struct.unpack(r"<%dI" %data_words, data)) & 0xFFFFFFFF

    def __sizeof(self, source_str):
        """Get size of source data.
        """
        size = 0
        info_list = [i for i in map(lambda x:x.strip(), source_str.split('+'))]
        for info_str in info_list:
            if info_str in self.build_info_dict:
                size += len(self.build_info_dict[info_str][0])
            else:
                self.LOG.info(f"Fail to get data {info_str}")
        return size

    def __crc32(self, source_str):
        """Calculate crc32 value of source data.
        """
        val = 0
        info_list = [i for i in map(lambda x:x.strip(), source_str.split('+'))]
        for info_str in info_list:
            if info_str in self.build_info_dict:
                val = binascii.crc32(self.build_info_dict[info_str][0], val)
            else:
                self.LOG.info(f"Fail to get data {info_str}")
        return val

    def __sum32(self, source_str):
        """Calculate accumulate value of source data.
        """
        val = 0
        info_list = [i for i in map(lambda x:x.strip(), source_str.split('+'))]
        for info_str in info_list:
            if info_str in self.build_info_dict:
                val += self._sum32(self.build_info_dict[info_str][0])
            else:
                self.LOG.info(f"Fail to get data {info_str}")
        return val & 0xFFFFFFFF

    def _openssl_operation(self, cmd_tuple, para_tuple):
        """Excute openssl command, build binary file.
        """
        if cmd_tuple[1] != len(para_tuple):
            self.LOG.error(f"Para number is NOT qualified, has {len(para_tuple)}, expect {cmd_tuple[1]}")

        cmd_str = cmd_tuple[0].format(*para_tuple)
        ret = self._run_shell_cmd(cmd_str)
        return 0 == ret and True or False

    def __extract_public_key_discard(self, asn1_str, key_lable):
        """Extract public key data from public key string(ASN.1 format).
        Current only support RSA and ECC public key extract.
        Have problem in older openssl version.
        """
        # filter begin, end, and new line
        str_list = asn1_str.splitlines()
        key_str = ''
        for lines in str_list:
            if '-----' not in lines:
                key_str += lines
            elif key_str:
                # encouter the second key boundary string
                break

        pubkey_der = binascii.a2b_base64(key_str)
        self.LOG.debug(asn1_str)
        self.LOG.debug(key_str)
        self.LOG.debug(binascii.hexlify(pubkey_der, ' '))
        key_binary = ''
        i = 0
        while i < len(pubkey_der):
            type, length = struct.unpack(r'<BB', pubkey_der[i:i+2])
            self.LOG.debug(f"DEC data type 0x{type:x}")
            i += 2
            if length > 0x80:
                count = length & 0x7F
                length = int(binascii.hexlify(pubkey_der[i:i+count]), 16)
                i += count
                self.LOG.debug(f"public key length {length:x}")

            # RSA pubkey or ECC pubkey
            if key_lable == type:
                key_binary = pubkey_der[i:i+length]
                break
            elif type < 0x30:
                # not sequence header
                i += length

        self.LOG.debug(binascii.hexlify(key_binary, ' '))
        # strip 0 in the MSB
        return key_binary.lstrip(b'\x00')

    def __extract_public_key(self, key_info_str, key_lable):
        """Extract public key data from public key info.
        """
        # hex string format: label(:HH){n}
        hex_convert = lambda x : b''.join([binascii.unhexlify(i.strip())
            for i in x[1:].split(":")])

        pub_key_p = re.compile(r"%s((:\s*\w\w){8,})" %key_lable, re.M)
        pub_obj = pub_key_p.search(key_info_str)
        key_binary = hex_convert(pub_obj.groups()[0])
        if 0 == key_binary[0]:
            key_binary = key_binary[1:]

        self.LOG.debug(key_info_str)
        # self.LOG.debug(key_binary)
        return key_binary

    def __get_hash_config(self, info_str):
        """Parse config string, get hash type, string example:
        SHA256, SHA256+RSA2048
        return info tuple: hash_type
        """
        info_list = [i.lower() for i in map(lambda x:x.strip(), info_str.split('+'))]
        for hash_type in info_list:
            if hash_type in self.hash_type_tuple:
                return ('HASH', hash_type)
        return (None, None)

    def __get_al_config(self, info_str):
        """Parse config string, get algorithm type and config, string example:
        RSA2048, RSA-2048, SHA256+RSA2048, SHA256+ECCprime256v1
        return info tuple: (algorithm, subtype)
        """
        info_list = [i.upper() for i in map(lambda x:x.strip(), info_str.split('+'))]
        for al_str in info_list:
            for key in self.encrypt_method_dict:
                if al_str.startswith(key):
                    al_para = al_str.split(key)[1].strip().lower()
                    while al_para.startswith('-'):
                        al_para = al_para[1:]
                    if al_para.isdigit():
                        al_para = int(al_para)
                    return (key, al_para)

        return None

    def __get_source_data(self, source_str):
        """Get source data from source file or from prevous processed data.
        return binary data.
        """
        info_list = [i for i in map(lambda x:x.strip(), source_str.split('+'))]
        binary_data = b''
        for source_file in info_list:
            if not source_file or not isinstance(source_file, str):
                self.LOG.error(f"Illegal input source {source_file}")
                continue

            input_file = os.path.join(self.input_path, source_file)
            if source_file in self.build_info_dict:
                # Get prevous processed binary data
                binary_data += self.build_info_dict[source_file][0]
            elif os.path.isfile(input_file):
                with open(input_file, 'rb') as f:
                    binary_data += f.read()
            else:
                self.LOG.info(f"Fail to find source data {source_file}")

        return binary_data

    def __generate_key_filename(self, key_name):
        """Generate public/private key file name with the key folder.
        """
        key_folder = os.path.join(self.input_path, 'key')
        if not os.path.isdir(key_folder):
            os.mkdir(key_folder)

        key_filename = os.path.join(key_folder, f"{key_name}.key")
        # key_file = open(key_filename, 'w+b')
        # self.key_file_list.append(key_file)
        return key_filename

    def __build_structure_data(self, structure_list):
        """Build binary data according to structure definition, string pattern:
        name, value(string or data), size
        """
        name = 'unkown'
        binary_data = b''
        for info_str in structure_list:
            if isinstance(info_str, str):
                info_list = [i for i in map(lambda x:x.strip(), info_str.split(','))]
                if len(info_list) < 3:
                    self.LOG.error(f"{info_str} MUST has 3 items at least")
                    continue

                label, value, size = info_list[:3]
                size = int(size, 0)
                self.LOG.debug(f"label {label} value {value} size {size}")
                key_obj = self.key_method_p.search(value)
                self.LOG.debug(f"key_obj {key_obj}")
                if label == 'name':
                    name = value
                elif label == 'pad':
                    binary_data += self._itob(int(value, 0) & 0xFF) * size
                elif key_obj:
                    key_groups = key_obj.groups()
                    method = self.key_method_dict[key_groups[0].lower()]
                    binary_data += self._align_binary_data(self._itob(method(key_groups[1])), size)
                else:
                    binary_data += self._struct_member_convert(value, size)
            elif isinstance(info_str, dict):
                # nest data definition
                for key, info in info_str.items():
                    if key in self.data_type_op_dict:
                        binary_data += self.data_type_op_dict[key](info)[1]
                    else:
                        self.LOG.error(f"NOT support structure subitem {key}")
            else:
                self.LOG.error(f"NOT support config {info_str}")

        self.LOG.debug(f"Has {len(binary_data)}Bytes in structure {name}")
        self.build_info_dict[name] = (binary_data, )
        return name, binary_data

    def __build_public_key_data(self, key_info_dict):
        """Generate public/private key pair according to asymmetric algorithm,
        save public key data
        """
        name = key_info_dict.get("name", "publickey")
        if name in self.build_info_dict:
            self.LOG.debug("Better NOT re-produce key pair {name}")
            return name, self.build_info_dict[name][0]

        align = key_info_dict.get("align", 1)
        # algorithm config info: asymmetric encryption type, asymmetric algorithm parameter
        al_info_tuple = self.__get_al_config(key_info_dict.get(self.al_key_str, "RSA2048"))
        if not al_info_tuple or 2 != len(al_info_tuple):
            self.LOG.error(f"Illegal algorithm {al_info_tuple} while get key config")
            return name, b''

        key_source = key_info_dict.get("source", '')
        key_file_name = os.path.join(self.input_path, key_source)
        self.LOG.debug(key_file_name)

        prvkey_filename, pubkey_filename = '', ''
        # private key, public key, public key info
        if key_source and os.path.isfile(key_file_name):
            with open(key_file_name, 'r', encoding = 'utf-8') as f:
                first_line_str = f.readline()

            if 'PRIVATE KEY' in first_line_str:
                prvkey_filename = key_file_name
            elif 'PUBLIC KEY' in first_line_str:
                pubkey_filename = key_file_name  
            else:
                self.LOG.error(f"NO valid private or public key in file {key_file_name}!")
                return name, b''

        method, cmd_dict, pubkey_str = self.encrypt_method_dict[al_info_tuple[0]]
        if not pubkey_filename:
            if not prvkey_filename:
                prvkey_filename = self.__generate_key_filename(f"{name}_prv")
                # generate prvate key if NO private key assign
                method(cmd_dict["private"], (prvkey_filename, al_info_tuple[1]))

            # generate public key from prvate key
            pubkey_filename = self.__generate_key_filename(f"{name}_pub")
            method(cmd_dict["public"], (prvkey_filename, pubkey_filename))

        # generate public key info from public key
        pubkey_info_file = tempfile.NamedTemporaryFile(delete = False)
        self.temp_file_list.append(pubkey_info_file)
        method(cmd_dict["pubparse"], (pubkey_filename, pubkey_info_file.name))

        self.build_info_dict[name + '_key'] = (prvkey_filename, pubkey_filename)
        pubkey_info_file.seek(0)
        binary_data = self.__extract_public_key(pubkey_info_file.read().decode('utf-8'), pubkey_str)
        binary_data = self._align_binary_data(binary_data, align)
        self.build_info_dict[name] = (binary_data, )
        return name, binary_data

    def __build_signature_data(self, sign_info_dict):
        """Build signature for selected data.
        """
        name = sign_info_dict.get("name", "signature")
        align = sign_info_dict.get("align", 1)
        al_info_tuple = self.__get_al_config(sign_info_dict.get(self.al_key_str, "RSA2048"))
        self.LOG.debug(al_info_tuple)
        if not al_info_tuple or 2 != len(al_info_tuple):
            self.LOG.error(f"Illegal algorithm {al_info_tuple} while get algorithm config")
            return name, b''

        hash_info_tuple = self.__get_hash_config(sign_info_dict.get(self.al_key_str, "SHA256"))
        key = sign_info_dict.get("key", "keypair") + '_key'
        if key not in self.build_info_dict:
            self.LOG.error(f"NO matched signature key {key}")
            return name, b''

        source_file = sign_info_dict.get("source", '')
        temp_file = tempfile.NamedTemporaryFile(delete = False)
        self.temp_file_list.append(temp_file)
        temp_file.write(self.__get_source_data(source_file))
        temp_file.close()

        prvkey_file_name = self.build_info_dict[key][0]
        sign_file = tempfile.NamedTemporaryFile(delete = False)
        self.temp_file_list.append(sign_file)
        method, cmd_dict = self.encrypt_method_dict[al_info_tuple[0]][:2]
        self.LOG.debug(hash_info_tuple)
        method(cmd_dict["sign"], (prvkey_file_name, temp_file.name, sign_file.name, hash_info_tuple[1]))

        sign_file.seek(0)
        binary_data = self._align_binary_data(sign_file.read(), align)
        self.build_info_dict[name] = (binary_data, )
        return name, binary_data

    def __build_hash_data(self, hash_info_dict):
        """Build hash data.
        """
        name = hash_info_dict.get("name", "hash")
        align = hash_info_dict.get("align", 1)
        hash_info_tuple = self.__get_hash_config(hash_info_dict.get(self.al_key_str, "SHA256"))
        if not hash_info_tuple or 2 != len(hash_info_tuple):
            self.LOG.error(f"Illegal hash type {hash_info_tuple}")
            return name, b''

        source_file = hash_info_dict.get("source", '')
        temp_file = tempfile.NamedTemporaryFile(delete = False)
        self.temp_file_list.append(temp_file)
        temp_file.write(self.__get_source_data(source_file))
        temp_file.close()

        hash_file = tempfile.NamedTemporaryFile(delete = False)
        self.temp_file_list.append(hash_file)
        self.LOG.debug(hash_info_tuple)
        self._openssl_operation(self.hash_cmd_tuple, (temp_file.name, hash_file.name, hash_info_tuple[1]))

        hash_file.seek(0)
        binary_data = self._align_binary_data(hash_file.read(), align)
        self.build_info_dict[name] = (binary_data, )
        return name, binary_data

    def __build_file_data(self, file_info_dict):
        """Get file data through file name.
        """
        name = file_info_dict.get("name", "empty_file")
        binary_data = self.__get_source_data(file_info_dict.get("source", ""))
        align = file_info_dict.get("align", 1)
        binary_data = self._align_binary_data(binary_data, align)
        self.build_info_dict[name] = (binary_data, )
        return name, binary_data

    def extract_yaml_config(self, yaml_file):
        """Extract configuration from XML file.
        """
        if not os.path.isfile(yaml_file):
            self.LOG.error("Config file %s NOT exist" %yaml_file)
            return False

        with open(yaml_file, 'r', encoding = 'utf-8') as f:
            config_info_dict = yaml.load(f, Loader = yaml.FullLoader)
        # self.LOG.debug(json.dumps(config_info_dict, indent = 2))
        self.input_path = os.path.dirname(yaml_file)
        return config_info_dict

    def extract_config(self, json_file):
        """Extract configuration from JSON file.
        """
        if not os.path.isfile(json_file):
            self.LOG.error("Config file %s NOT exist" %json_file)
            return False

        with open(json_file, 'r', encoding = 'utf-8') as f:
            config_info_dict = json.loads(f.read())
        # self.LOG.debug(json.dumps(config_info_dict, indent = 2))
        if "_comment" in config_info_dict:
            config_info_dict.pop("_comment")
        self.input_path = os.path.dirname(json_file)
        return config_info_dict

    def verify_config(self, config_dict):
        """verify image config dictionary.
        """
        if self.info_key_str in config_dict:
            for key, value in config_dict[self.info_key_str].items():
                self.LOG.info(f"{key} : {value}")

        if self.image_key not in config_dict:
            self.LOG.error(f"NO {self.image_key} in YML file")
            return False

        image_list = config_dict[self.image_key]
        self.LOG.debug(json.dumps(image_list, indent = 2))
        if not isinstance(image_list, list):
            self.LOG.error(f"{self.image_key} config in YML file is illegal, MUST be a list")
            return False

        self.LOG.debug(f"Has {len(image_list)} modules in image")
        for info_dict in image_list:
            if not isinstance(info_dict, dict):
                break

            if self.image_module_key not in info_dict or self.image_data_key not in info_dict:
                self.LOG.error(f"MUST have {self.image_module_key} \
                    and {self.image_data_key} in image configuration")

            self.LOG.debug(f"Has {len(info_dict[self.image_data_key])} data section in {info_dict[self.image_module_key]}")

        return True

    def release_temp_file(self):
        for temp_file in self.temp_file_list:
            temp_file.close()
            os.remove(temp_file.name)
        for temp_file in self.key_file_list:
            temp_file.close()

    def preprocess_data(self, image_list):
        """Pre-parse config info, build data.
        """
        # DO NOT print error message in this pre-process routine
        self.LOG.set_debug_level('error')
        for image_dict in image_list:
            for data_info_dict in image_dict[self.image_data_key]:
                for data_type, info in data_info_dict.items():
                    if data_type in self.data_type_op_dict:
                        _data_name, _data = self.data_type_op_dict[data_type](info)
        self.LOG.set_debug_level()
        return True

    def build_image(self, config_dict, output_file):
        with open(output_file, 'wb') as f:
            image_list = config_dict[self.image_key]
            # build data in case some data used before it's build
            self.preprocess_data(image_list)
            for image_dict in image_list:
                self.LOG.debug(f"Build image data of module {image_dict[self.image_module_key]}")
                for data_info_dict in image_dict[self.image_data_key]:
                    for data_type, info in data_info_dict.items():
                        if data_type in self.data_type_op_dict:
                            data_name, data = self.data_type_op_dict[data_type](info)
                            self.LOG.debug(f"Build {len(data)}Bytes data of image {data_name}")
                            f.write(data)

        self.release_temp_file()
        return True



def main(argv):
    parser = argparse.ArgumentParser(
        description='Parse JSON config file, collect related files, and build image file.',
    )
    parser.add_argument('-c',   dest = 'json_file',     required = True,    help = 'configuration json file')
    parser.add_argument('-o',   dest = 'output_file',   default = 'img.bin', help = 'output file')

    args = parser.parse_args()
    json_file = args.json_file
    output_file = args.output_file

    log = common_decorator.Logger()
    # log = common_decorator.Logger(clevel = 'DEBUG')
    image = ImageBinary(log)
    config_info_dict = image.extract_config(json_file)
    if config_info_dict and image.verify_config(config_info_dict):
        image.build_image(config_info_dict, output_file)


if __name__ == '__main__':
    main(sys.argv[1:])
