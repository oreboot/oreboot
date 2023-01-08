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
        let mut dev = Some(root_dev);

        while let Some(d) = dev {
            /* Ignore disabled devices. */
            if d.fields.enabled() == 0 {
                continue;
            }

            let mut res = d.resource_list;

            while let Some(r) = res {
                /* If it isn't the right kind of resource ignore it. */
                if r.flags & type_mask != res_type {
                    continue;
                }

                /* If it is a subtractive resource ignore it. */
                if (r.flags & IORESOURCE_SUBTRACTIVE) != 0 {
                    continue;
                }

                /* If the resource is not assigned ignore it. */
                if (r.flags & IORESOURCE_ASSIGNED) == 0 {
                    continue;
                }

                search(self, d, r)?;
                res = r.next;
            }
            dev = d.next;
        }
        Ok(())
    }
}
