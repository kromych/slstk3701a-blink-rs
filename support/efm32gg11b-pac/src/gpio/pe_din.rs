#[doc = "Register `PE_DIN` reader"]
pub type R = crate::R<PE_DIN_SPEC>;
#[doc = "Field `DIN` reader - Data in"]
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port Data in Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_din::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_DIN_SPEC;
impl crate::RegisterSpec for PE_DIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_din::R`](R) reader structure"]
impl crate::Readable for PE_DIN_SPEC {}
#[doc = "`reset()` method sets PE_DIN to value 0"]
impl crate::Resettable for PE_DIN_SPEC {
    const RESET_VALUE: u32 = 0;
}
