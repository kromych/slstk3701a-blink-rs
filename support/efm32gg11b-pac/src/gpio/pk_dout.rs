#[doc = "Register `PK_DOUT` reader"]
pub type R = crate::R<PK_DOUT_SPEC>;
#[doc = "Register `PK_DOUT` writer"]
pub type W = crate::W<PK_DOUT_SPEC>;
#[doc = "Field `DOUT` reader - Data Out"]
pub type DOUT_R = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<PK_DOUT_SPEC> {
        DOUT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pk_dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pk_dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PK_DOUT_SPEC;
impl crate::RegisterSpec for PK_DOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pk_dout::R`](R) reader structure"]
impl crate::Readable for PK_DOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pk_dout::W`](W) writer structure"]
impl crate::Writable for PK_DOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PK_DOUT to value 0"]
impl crate::Resettable for PK_DOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
