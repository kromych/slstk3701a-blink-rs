#[doc = "Register `PK_DOUTTGL` writer"]
pub type W = crate::W<PkDouttglSpec>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DouttglW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DouttglW<'_, PkDouttglSpec> {
        DouttglW::new(self, 0)
    }
}
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pk_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkDouttglSpec;
impl crate::RegisterSpec for PkDouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pk_douttgl::W`](W) writer structure"]
impl crate::Writable for PkDouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PK_DOUTTGL to value 0"]
impl crate::Resettable for PkDouttglSpec {}
