pub const NSECS_PER_SEC: u64 = 1000000000;
pub const USECS_PER_SEC: u64 = 1000000;
pub const MSECS_PER_SEC: u64 = 1000;
pub const USECS_PER_MSEC: u64 = USECS_PER_SEC / MSECS_PER_SEC;

/* The time structures are defined to be a representation of the time since
 * coreboot started executing one of its stages. The reason for using structures
 * is to allow for changes in the future. The structures' details are exposed
 * so that the compiler can allocate space on the stack and use in other
 * structures. In other words, accessing any field within this structure
 * outside of the core timer code is not supported. */

#[repr(C)]
#[derive(PartialEq)]
pub struct MonoTime {
    pub microseconds: u64,
}

impl MonoTime {
    pub const fn new() -> Self {
        Self { microseconds: 0 }
    }

    pub fn add_usecs(&mut self, us: u64) {
        self.microseconds += us;
    }

    /**
     * Obtain the current monotonic time. The assumption is that the time counts
     * up from the value 0 with value 0 being the point when the timer was
     * initialized.  Additionally, the timer is assumed to only be valid for the
     * duration of the boot.
     *
     * Note that any implementations of timer_monotonic_get()
     * need to ensure its timesource does not roll over within 10 secs. The reason
     * is that the time between calls to timer_monotonic_get() may be on order
     * of 10 seconds. */
    // TODO: there is no generic implementation, monotonic timer is completely hardware dependent
    pub fn monotonic_get(&mut self) {
        unimplemented!("needs hardware-specific implementation(s)");
    }

    /// Compare two absolute times:
    /// Return -1, 0, or 1 if t1 is <, =, or > t2, respectively.
    pub fn cmp(&self, oth: &Self) -> i8 {
        if self.microseconds == oth.microseconds {
            return 0;
        }
        if self.microseconds < oth.microseconds {
            return -1;
        }
        return 1;
    }

    pub fn before(&self, oth: &Self) -> bool {
        self.cmp(oth) < 0
    }

    pub fn diff_microseconds(&self, oth: &Self) -> i64 {
        (self.microseconds as i64) - (oth.microseconds as i64)
    }
}

pub struct Stopwatch {
    pub start: MonoTime,
    pub current: MonoTime,
    pub expires: MonoTime,
}

impl Stopwatch {
    pub const fn new() -> Self {
        Self {
            start: MonoTime::new(),
            current: MonoTime::new(),
            expires: MonoTime::new(),
        }
    }

    pub fn init(&mut self) {
        // FIXME: setup custom config for google and timer code
        //if config!(HAVE_MONOTONIC_TIMER) {
        //    self.start.timer_monotonic_get();
        //} else {
        self.start.microseconds = 0;
        // }

        self.expires.microseconds = self.start.microseconds;
        self.current.microseconds = self.expires.microseconds;
    }

    pub fn init_usecs_expire(&mut self, us: u64) {
        self.init();
        self.expires.add_usecs(us);
    }

    pub fn init_msecs_expire(&mut self, ms: u64) {
        self.init_usecs_expire(USECS_PER_MSEC as u64 * ms);
    }

    pub fn expired(&mut self) -> bool {
        self.tick();
        !self.current.before(&self.expires)
    }

    pub fn tick(&mut self) {
        // TODO: setup config for monotonic timer
        //if config!(HAVE_MONOTONIC_TIMER) {
        //    sw.current.monotonic_get();
        //} else {
        self.current.microseconds = 0;
        //}
    }

    /// Tick and check the stopwatch as long as it has not expired.
    pub fn wait_until_expired(&mut self) {
        loop {
            if self.expired() {
                break;
            }
        }
    }

    /// Return number of microseconds since starting the stopwatch.
    pub fn duration_usecs(&mut self) -> i64 {
        /*
         * If the stopwatch hasn't been ticked (current == start) tick
         * the stopwatch to gather the accumulated time.
         */
        if self.start == self.current {
            self.tick();
        }

        self.start.diff_microseconds(&self.current)
    }

    pub fn duration_msecs(&mut self) -> i64 {
        self.duration_usecs() / (USECS_PER_MSEC as i64)
    }
}

pub fn udelay(mut usec: u32) {
    let mut sw = Stopwatch::new();

    /*
     * As the timer granularity is in microseconds pad the
     * requested delay by one to get at least >= requested usec delay.
     */
    usec += 1;

    // TODO: add thread_yield_microseconds impl
    //if !thread_yield_microseconds(usec) {
    //      return;
    //}

    sw.init_usecs_expire(usec as u64);
    sw.wait_until_expired();
}
