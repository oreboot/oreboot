use crate::{
    resource::{Resource, IORESOURCE_ASSIGNED, IORESOURCE_SUBTRACTIVE},
    Device,
};

pub trait GlobalSearch {
    type Error;

    fn search_global_resources(
        &mut self,
        root_dev: &Device,
        type_mask: u32,
        res_type: u32,
        search: fn(&mut Self, &Device, &Resource) -> Result<(), Self::Error>,
    ) -> Result<(), Self::Error> {
        unsafe {
            let mut dev = root_dev as *const Device;

            while dev != core::ptr::null() {
                /* Ignore disabled devices. */
                if (*dev).fields.enabled() == 0 {
                    continue;
                }

                let mut res = (*dev).resource_list;

                while res != core::ptr::null() {
                    /* If it isn't the right kind of resource ignore it. */
                    if (*res).flags & type_mask != res_type {
                        continue;
                    }

                    /* If it is a subtractive resource ignore it. */
                    if ((*res).flags & IORESOURCE_SUBTRACTIVE) != 0 {
                        continue;
                    }

                    /* If the resource is not assigned ignore it. */
                    if ((*res).flags & IORESOURCE_ASSIGNED) == 0 {
                        continue;
                    }

                    search(self, &*dev, &*res)?;
                    res = (*res).next;
                }
                dev = (*dev).next;
            }
            Ok(())
        }
    }
}
