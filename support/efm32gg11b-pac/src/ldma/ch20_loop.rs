#[doc = "Register `CH20_LOOP` reader"]
pub type R = crate::R<CH20_LOOP_SPEC>;
#[doc = "Register `CH20_LOOP` writer"]
pub type W = crate::W<CH20_LOOP_SPEC>;
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_R = crate::FieldReader;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LOOPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    #[must_use]
    pub fn loopcnt(&mut self) -> LOOPCNT_W<CH20_LOOP_SPEC> {
        LOOPCNT_W::new(self, 0)
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
#[doc = "Channel Loop Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch20_loop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch20_loop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH20_LOOP_SPEC;
impl crate::RegisterSpec for CH20_LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch20_loop::R`](R) reader structure"]
impl crate::Readable for CH20_LOOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch20_loop::W`](W) writer structure"]
impl crate::Writable for CH20_LOOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH20_LOOP to value 0"]
impl crate::Resettable for CH20_LOOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
