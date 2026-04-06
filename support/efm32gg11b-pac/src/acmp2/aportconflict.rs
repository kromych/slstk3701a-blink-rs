#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<AportconflictSpec>;
#[doc = "Field `APORT0XCONFLICT` reader - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
pub type Aport0xconflictR = crate::BitReader;
#[doc = "Field `APORT0YCONFLICT` reader - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
pub type Aport0yconflictR = crate::BitReader;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type Aport1xconflictR = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type Aport1yconflictR = crate::BitReader;
#[doc = "Field `APORT2XCONFLICT` reader - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
pub type Aport2xconflictR = crate::BitReader;
#[doc = "Field `APORT2YCONFLICT` reader - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
pub type Aport2yconflictR = crate::BitReader;
#[doc = "Field `APORT3XCONFLICT` reader - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
pub type Aport3xconflictR = crate::BitReader;
#[doc = "Field `APORT3YCONFLICT` reader - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
pub type Aport3yconflictR = crate::BitReader;
#[doc = "Field `APORT4XCONFLICT` reader - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
pub type Aport4xconflictR = crate::BitReader;
#[doc = "Field `APORT4YCONFLICT` reader - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
pub type Aport4yconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0xconflict(&self) -> Aport0xconflictR {
        Aport0xconflictR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0yconflict(&self) -> Aport0yconflictR {
        Aport0yconflictR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> Aport1xconflictR {
        Aport1xconflictR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> Aport1yconflictR {
        Aport1yconflictR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2xconflict(&self) -> Aport2xconflictR {
        Aport2xconflictR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2yconflict(&self) -> Aport2yconflictR {
        Aport2yconflictR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3xconflict(&self) -> Aport3xconflictR {
        Aport3xconflictR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3yconflict(&self) -> Aport3yconflictR {
        Aport3yconflictR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4xconflict(&self) -> Aport4xconflictR {
        Aport4xconflictR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4yconflict(&self) -> Aport4yconflictR {
        Aport4yconflictR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "APORT Conflict Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportconflict::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AportconflictSpec;
impl crate::RegisterSpec for AportconflictSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for AportconflictSpec {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for AportconflictSpec {}
