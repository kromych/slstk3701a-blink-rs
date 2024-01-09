#[doc = "Register `MODULEID` reader"]
pub type R = crate::R<MODULEID_SPEC>;
#[doc = "Field `CONF` reader - Configuration ID Number"]
pub type CONF_R = crate::FieldReader;
#[doc = "Field `MODULEID` reader - Module/Revision ID Number"]
pub type MODULEID_R = crate::FieldReader<u16>;
#[doc = "Field `FIXPATCH` reader - Fix/patch Number"]
pub type FIXPATCH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Configuration ID Number"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - Module/Revision ID Number"]
    #[inline(always)]
    pub fn moduleid(&self) -> MODULEID_R {
        MODULEID_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Fix/patch Number"]
    #[inline(always)]
    pub fn fixpatch(&self) -> FIXPATCH_R {
        FIXPATCH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moduleid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODULEID_SPEC;
impl crate::RegisterSpec for MODULEID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moduleid::R`](R) reader structure"]
impl crate::Readable for MODULEID_SPEC {}
#[doc = "`reset()` method sets MODULEID to value 0x0200"]
impl crate::Resettable for MODULEID_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
