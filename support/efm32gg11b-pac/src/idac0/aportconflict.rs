#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<AportconflictSpec>;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type Aport1xconflictR = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type Aport1yconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> Aport1xconflictR {
        Aport1xconflictR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> Aport1yconflictR {
        Aport1yconflictR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportconflict::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AportconflictSpec;
impl crate::RegisterSpec for AportconflictSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for AportconflictSpec {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for AportconflictSpec {}
