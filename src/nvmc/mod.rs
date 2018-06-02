#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 256usize],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    #[doc = "0x508 - Register for erasing a page in code area"]
    pub erasepage: ERASEPAGE,
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for erasing user information configuration registers"]
    pub eraseuicr: ERASEUICR,
    _reserved2: [u8; 40usize],
    #[doc = "0x540 - I-code cache configuration register."]
    pub icachecnf: ICACHECNF,
    _reserved3: [u8; 4usize],
    #[doc = "0x548 - I-code cache hit counter."]
    pub ihit: IHIT,
    #[doc = "0x54c - I-code cache miss counter."]
    pub imiss: IMISS,
}
#[doc = "Ready flag"]
pub struct READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing a page in code area"]
pub struct ERASEPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory"]
pub struct ERASEALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - Register for erasing a page in code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "Register for erasing user information configuration registers"]
pub struct ERASEUICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "I-code cache configuration register."]
pub struct ICACHECNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache configuration register."]
pub mod icachecnf;
#[doc = "I-code cache hit counter."]
pub struct IHIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache hit counter."]
pub mod ihit;
#[doc = "I-code cache miss counter."]
pub struct IMISS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-code cache miss counter."]
pub mod imiss;
