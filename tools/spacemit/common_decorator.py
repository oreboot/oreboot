# -*- coding: utf-8 -*-
import time
import os
import sys
import fnmatch
import re
import functools
import argparse
import traceback
import logging

class Logger(object):
    def __init__(self, clevel = 'debug'):
        self.logger = logging.Logger(__name__)
        # self.logger = logging.getLogger(__name__)
        self.logger.setLevel(logging.DEBUG)

        self.LOGLEVEL = {
            'NOTSET': logging.NOTSET,   'DEBUG'     : logging.DEBUG,
            'INFO'  : logging.INFO,     'WARNING'   : logging.WARNING,
            'ERROR' : logging.ERROR,    'CRITICAL'  : logging.CRITICAL
        }
        self.debug      = self.logger.debug
        self.info       = self.logger.info
        self.warning    = self.logger.warning
        self.error      = self.logger.error
        self.critical   = self.logger.critical
        self.exception  = self.logger.exception

        self.stream_handler = None
        self.file_handler = None
        self.set_debug_level(clevel)

    def set_debug_level(self, clevel = 'debug'):
        clevel = clevel.upper()
        clevel = self.LOGLEVEL.get(clevel, logging.DEBUG)
        if clevel < logging.INFO:
            formatter = logging.Formatter('[%(module)s]: (%(lineno)d) (%(asctime)s) %(message)s')
        else:
            formatter = logging.Formatter('(%(asctime)s) %(message)s')

        # 设置CMD日志
        if self.stream_handler is not None:
            self.logger.removeHandler(self.stream_handler)
        sh = logging.StreamHandler()
        sh.setFormatter(formatter)
        sh.setLevel(clevel)
        self.logger.addHandler(sh)
        self.stream_handler = sh
        return True

    def set_file_debug_level(self, flevel = 'debug', path = None):
        if path:
            flevel = flevel.upper()
            flevel = self.LOGLEVEL.get(flevel, logging.DEBUG)
            if flevel < logging.INFO:
                formatter = logging.Formatter('[%(module)s]: (%(lineno)d) (%(asctime)s) %(message)s')
            else:
                formatter = logging.Formatter('(%(asctime)s) %(message)s')

            # 设置文件日志
            if self.file_handler is not None:
                self.logger.removeHandler(self.file_handler)
            fh = logging.FileHandler(path)
            fh.setFormatter(formatter)
            fh.setLevel(flevel)
            self.logger.addHandler(fh)
            self.file_handler = fh
            return True
        return False


def time_consume(func):
    @functools.wraps(func)
    def wrapper(*args, **kw):
        try:
            stamp_a = time.time()
            ret = func(*args, **kw)
            stamp_b = time.time()
            print("%s consume(second): %.3f." %(func.__name__, stamp_b - stamp_a))
            return ret
        except Exception as e:
            print('%s(%s)' %(type(e), e))
            # traceback.print_exc()
            return -1
    return wrapper


def except_wrapper(func):
    @functools.wraps(func)
    def wrapper(*args, **kw):
        try:
            return func(*args, **kw)
        except Exception as e:
            print('%s(%s)' %(type(e), e))
            traceback.print_exc()
            return -1
    return wrapper


def search_files(path_list, suffix_pattern_list):
    """search for file with suffix in suffix_list
    If suffix_list is empty, return all file in the path.
    return: matched file list iterator
    """
    # matches = []
    # print("search path: %r, pattern: %r" %(path_list, suffix_pattern_list))
    hiden_dir_p = re.compile(r'^\.\w+')
    for path in path_list:
        for root, dirnames, filenames in os.walk(path):
            for suffix_pattern in suffix_pattern_list:
                relative_path = os.path.relpath(root, path)
                if not hiden_dir_p.match(relative_path):
                    for filename in fnmatch.filter(filenames, r'%s' %suffix_pattern):
                        # matches.append(os.path.join(root, filename))
                        yield os.path.join(root, filename)

    # return matches


def PreparseConfig(input_file):
    """Preprocess input file, filter out all comment line string.
    """
    config_list = []

    if os.path.isfile(input_file):
        with open(input_file, 'r') as p_file:
            str             = p_file.read()
            line_str_list   = str.split('\n')

            for line_str in line_str_list:
                line_str = line_str.strip()
                # filter out comment string and empty line
                if line_str and not re.search(r'^\s*($|//|#)', line_str):
                    config_list.append(line_str)

    return config_list


def main(argv):
    parser = argparse.ArgumentParser(
        description='Extract interface with key, and generate link script.',
    )
    parser.add_argument('-i',   dest = 'input_path',    nargs = '+',    required = True,    help = 'search path')

    args = parser.parse_args()
    src_path    = args.input_path

    path_list = []
    for search_path in src_path:
        if os.path.exists(search_path):
            path_list.append(search_path)
        else:
            print("%s is NOT a file path !" %search_path)

    file_list_iter  = search_files(path_list, ['*.c'])
    for input_file in file_list_iter:
        PreparseConfig(input_file)


if __name__ == '__main__':
    main(sys.argv[1:])

