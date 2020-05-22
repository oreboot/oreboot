/* SPDX-License-Identifier: GPL-2.0-only */

#include <acpi/acpigen.h>
#if CONFIG(GENERIC_GPIO_LIB)
#include <gpio.h>
#endif
#include "chromeos.h"

void chromeos_acpi_gpio_generate(const struct cros_gpio *gpios, size_t num)
{
	size_t i;
	int gpio_num;

	acpigen_write_scope("\\");
	acpigen_write_name("OIPG");

	acpigen_write_package(num);
	for (i = 0; i < num; i++) {
		acpigen_write_package(4);
		acpigen_write_integer(gpios[i].type);
		acpigen_write_integer(gpios[i].polarity);
		gpio_num = gpios[i].gpio_num;
#if CONFIG(GENERIC_GPIO_LIB)
		/* Get ACPI pin from GPIO library if available */
		if (gpios[i].gpio_num != CROS_GPIO_VIRTUAL)
			gpio_num = gpio_acpi_pin(gpio_num);
#endif
		acpigen_write_integer(gpio_num);
		acpigen_write_string(gpios[i].device);
		acpigen_pop_len();
	}
	acpigen_pop_len();

	acpigen_pop_len();
}

void chromeos_dsdt_generator(const struct device *dev)
{
	mainboard_chromeos_acpi_generate();
}
