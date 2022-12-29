#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clkstat: CLKSTAT,
    #[doc = "0x04 - CLK Set Register"]
    pub clkset: CLKSET,
    #[doc = "0x08 - CLK Clear Register"]
    pub clkclr: CLKCLR,
    #[doc = "0x0c - System Clock Control Register"]
    pub sysclkcr: SYSCLKCR,
    #[doc = "0x10 - CPU Clock Control Register"]
    pub cpuclkcr: CPUCLKCR,
    #[doc = "0x14 - Peripheral Bus Clock Control Register"]
    pub pbclkcr: PBCLKCR,
    #[doc = "0x18 - USB Clock Control Register"]
    pub usbclkcr: USBCLKCR,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - CCU Clock Control Register"]
    pub ccuclkcr: CCUCLKCR,
    #[doc = "0x24 - WDT Clock Control Register"]
    pub wdtclkcr: WDTCLKCR,
    #[doc = "0x28 - External Clock Control"]
    pub extclkcr: EXTCLKCR,
    #[doc = "0x2c - Multi-Link Clock Control"]
    pub mlinkclkcr: MLINKCLKCR,
    #[doc = "0x30 - Sleep Control Register"]
    pub sleepcr: SLEEPCR,
    #[doc = "0x34 - Deep Sleep Control Register"]
    pub dsleepcr: DSLEEPCR,
    _reserved13: [u8; 0x08],
    #[doc = "0x40 - Peripheral 0 Clock Gating Status"]
    pub cgatstat0: CGATSTAT0,
    #[doc = "0x44 - Peripheral 0 Clock Gating Set"]
    pub cgatset0: CGATSET0,
    #[doc = "0x48 - Peripheral 0 Clock Gating Clear"]
    pub cgatclr0: CGATCLR0,
    #[doc = "0x4c - Peripheral 1 Clock Gating Status"]
    pub cgatstat1: CGATSTAT1,
    #[doc = "0x50 - Peripheral 1 Clock Gating Set"]
    pub cgatset1: CGATSET1,
    #[doc = "0x54 - Peripheral 1 Clock Gating Clear"]
    pub cgatclr1: CGATCLR1,
    #[doc = "0x58 - Peripheral 2 Clock Gating Status"]
    pub cgatstat2: CGATSTAT2,
    #[doc = "0x5c - Peripheral 2 Clock Gating Set"]
    pub cgatset2: CGATSET2,
    #[doc = "0x60 - Peripheral 2 Clock Gating Clear"]
    pub cgatclr2: CGATCLR2,
}
#[doc = "CLKSTAT (r) register accessor: an alias for `Reg<CLKSTAT_SPEC>`"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "Clock Status Register"]
pub mod clkstat;
#[doc = "CLKSET (w) register accessor: an alias for `Reg<CLKSET_SPEC>`"]
pub type CLKSET = crate::Reg<clkset::CLKSET_SPEC>;
#[doc = "CLK Set Register"]
pub mod clkset;
#[doc = "CLKCLR (w) register accessor: an alias for `Reg<CLKCLR_SPEC>`"]
pub type CLKCLR = crate::Reg<clkclr::CLKCLR_SPEC>;
#[doc = "CLK Clear Register"]
pub mod clkclr;
#[doc = "SYSCLKCR (rw) register accessor: an alias for `Reg<SYSCLKCR_SPEC>`"]
pub type SYSCLKCR = crate::Reg<sysclkcr::SYSCLKCR_SPEC>;
#[doc = "System Clock Control Register"]
pub mod sysclkcr;
#[doc = "CPUCLKCR (rw) register accessor: an alias for `Reg<CPUCLKCR_SPEC>`"]
pub type CPUCLKCR = crate::Reg<cpuclkcr::CPUCLKCR_SPEC>;
#[doc = "CPU Clock Control Register"]
pub mod cpuclkcr;
#[doc = "PBCLKCR (rw) register accessor: an alias for `Reg<PBCLKCR_SPEC>`"]
pub type PBCLKCR = crate::Reg<pbclkcr::PBCLKCR_SPEC>;
#[doc = "Peripheral Bus Clock Control Register"]
pub mod pbclkcr;
#[doc = "USBCLKCR (rw) register accessor: an alias for `Reg<USBCLKCR_SPEC>`"]
pub type USBCLKCR = crate::Reg<usbclkcr::USBCLKCR_SPEC>;
#[doc = "USB Clock Control Register"]
pub mod usbclkcr;
#[doc = "CCUCLKCR (rw) register accessor: an alias for `Reg<CCUCLKCR_SPEC>`"]
pub type CCUCLKCR = crate::Reg<ccuclkcr::CCUCLKCR_SPEC>;
#[doc = "CCU Clock Control Register"]
pub mod ccuclkcr;
#[doc = "WDTCLKCR (rw) register accessor: an alias for `Reg<WDTCLKCR_SPEC>`"]
pub type WDTCLKCR = crate::Reg<wdtclkcr::WDTCLKCR_SPEC>;
#[doc = "WDT Clock Control Register"]
pub mod wdtclkcr;
#[doc = "EXTCLKCR (rw) register accessor: an alias for `Reg<EXTCLKCR_SPEC>`"]
pub type EXTCLKCR = crate::Reg<extclkcr::EXTCLKCR_SPEC>;
#[doc = "External Clock Control"]
pub mod extclkcr;
#[doc = "MLINKCLKCR (rw) register accessor: an alias for `Reg<MLINKCLKCR_SPEC>`"]
pub type MLINKCLKCR = crate::Reg<mlinkclkcr::MLINKCLKCR_SPEC>;
#[doc = "Multi-Link Clock Control"]
pub mod mlinkclkcr;
#[doc = "SLEEPCR (rw) register accessor: an alias for `Reg<SLEEPCR_SPEC>`"]
pub type SLEEPCR = crate::Reg<sleepcr::SLEEPCR_SPEC>;
#[doc = "Sleep Control Register"]
pub mod sleepcr;
#[doc = "DSLEEPCR (rw) register accessor: an alias for `Reg<DSLEEPCR_SPEC>`"]
pub type DSLEEPCR = crate::Reg<dsleepcr::DSLEEPCR_SPEC>;
#[doc = "Deep Sleep Control Register"]
pub mod dsleepcr;
#[doc = "CGATSTAT0 (r) register accessor: an alias for `Reg<CGATSTAT0_SPEC>`"]
pub type CGATSTAT0 = crate::Reg<cgatstat0::CGATSTAT0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Status"]
pub mod cgatstat0;
#[doc = "CGATSET0 (w) register accessor: an alias for `Reg<CGATSET0_SPEC>`"]
pub type CGATSET0 = crate::Reg<cgatset0::CGATSET0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Set"]
pub mod cgatset0;
#[doc = "CGATCLR0 (w) register accessor: an alias for `Reg<CGATCLR0_SPEC>`"]
pub type CGATCLR0 = crate::Reg<cgatclr0::CGATCLR0_SPEC>;
#[doc = "Peripheral 0 Clock Gating Clear"]
pub mod cgatclr0;
#[doc = "CGATSTAT1 (r) register accessor: an alias for `Reg<CGATSTAT1_SPEC>`"]
pub type CGATSTAT1 = crate::Reg<cgatstat1::CGATSTAT1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Status"]
pub mod cgatstat1;
#[doc = "CGATSET1 (w) register accessor: an alias for `Reg<CGATSET1_SPEC>`"]
pub type CGATSET1 = crate::Reg<cgatset1::CGATSET1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Set"]
pub mod cgatset1;
#[doc = "CGATCLR1 (w) register accessor: an alias for `Reg<CGATCLR1_SPEC>`"]
pub type CGATCLR1 = crate::Reg<cgatclr1::CGATCLR1_SPEC>;
#[doc = "Peripheral 1 Clock Gating Clear"]
pub mod cgatclr1;
#[doc = "CGATSTAT2 (r) register accessor: an alias for `Reg<CGATSTAT2_SPEC>`"]
pub type CGATSTAT2 = crate::Reg<cgatstat2::CGATSTAT2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Status"]
pub mod cgatstat2;
#[doc = "CGATSET2 (w) register accessor: an alias for `Reg<CGATSET2_SPEC>`"]
pub type CGATSET2 = crate::Reg<cgatset2::CGATSET2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Set"]
pub mod cgatset2;
#[doc = "CGATCLR2 (w) register accessor: an alias for `Reg<CGATCLR2_SPEC>`"]
pub type CGATCLR2 = crate::Reg<cgatclr2::CGATCLR2_SPEC>;
#[doc = "Peripheral 2 Clock Gating Clear"]
pub mod cgatclr2;
