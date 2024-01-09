#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `AHBACT` reader - EBI Busy With AHB Transaction"]
pub type AHBACT_R = crate::BitReader;
#[doc = "Field `ECCACT` reader - EBI ECC Generation Active"]
pub type ECCACT_R = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 is Empty"]
pub type TFTPIXEL0EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 is Empty"]
pub type TFTPIXEL1EMPTY_R = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL0 is Full"]
pub type TFTPIXELFULL_R = crate::BitReader;
#[doc = "Field `DDACT` reader - EBI Busy With Direct Drive Transactions"]
pub type DDACT_R = crate::BitReader;
#[doc = "Field `TFTDDEMPTY` reader - EBI_TFTDD Register is Empty"]
pub type TFTDDEMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EBI Busy With AHB Transaction"]
    #[inline(always)]
    pub fn ahbact(&self) -> AHBACT_R {
        AHBACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - EBI ECC Generation Active"]
    #[inline(always)]
    pub fn eccact(&self) -> ECCACT_R {
        ECCACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL0 is Empty"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL1 is Empty"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EBI_TFTPIXEL0 is Full"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - EBI Busy With Direct Drive Transactions"]
    #[inline(always)]
    pub fn ddact(&self) -> DDACT_R {
        DDACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EBI_TFTDD Register is Empty"]
    #[inline(always)]
    pub fn tftddempty(&self) -> TFTDDEMPTY_R {
        TFTDDEMPTY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
