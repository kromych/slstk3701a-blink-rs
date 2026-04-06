#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `AHBACT` reader - EBI Busy With AHB Transaction"]
pub type AhbactR = crate::BitReader;
#[doc = "Field `ECCACT` reader - EBI ECC Generation Active"]
pub type EccactR = crate::BitReader;
#[doc = "Field `TFTPIXEL0EMPTY` reader - EBI_TFTPIXEL0 is Empty"]
pub type Tftpixel0emptyR = crate::BitReader;
#[doc = "Field `TFTPIXEL1EMPTY` reader - EBI_TFTPIXEL1 is Empty"]
pub type Tftpixel1emptyR = crate::BitReader;
#[doc = "Field `TFTPIXELFULL` reader - EBI_TFTPIXEL0 is Full"]
pub type TftpixelfullR = crate::BitReader;
#[doc = "Field `DDACT` reader - EBI Busy With Direct Drive Transactions"]
pub type DdactR = crate::BitReader;
#[doc = "Field `TFTDDEMPTY` reader - EBI_TFTDD Register is Empty"]
pub type TftddemptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EBI Busy With AHB Transaction"]
    #[inline(always)]
    pub fn ahbact(&self) -> AhbactR {
        AhbactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - EBI ECC Generation Active"]
    #[inline(always)]
    pub fn eccact(&self) -> EccactR {
        EccactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL0 is Empty"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> Tftpixel0emptyR {
        Tftpixel0emptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL1 is Empty"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> Tftpixel1emptyR {
        Tftpixel1emptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EBI_TFTPIXEL0 is Full"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TftpixelfullR {
        TftpixelfullR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - EBI Busy With Direct Drive Transactions"]
    #[inline(always)]
    pub fn ddact(&self) -> DdactR {
        DdactR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EBI_TFTDD Register is Empty"]
    #[inline(always)]
    pub fn tftddempty(&self) -> TftddemptyR {
        TftddemptyR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
