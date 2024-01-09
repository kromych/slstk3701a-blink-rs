#[doc = "Register `IF0IFC` writer"]
pub type W = crate::W<IF0IFC_SPEC>;
#[doc = "Field `MESSAGE` writer - Clear MESSAGE Interrupt Flag"]
pub type MESSAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear MESSAGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MESSAGE_W<IF0IFC_SPEC> {
        MESSAGE_W::new(self, 0)
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
#[doc = "Message Object Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF0IFC_SPEC;
impl crate::RegisterSpec for IF0IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`if0ifc::W`](W) writer structure"]
impl crate::Writable for IF0IFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IF0IFC to value 0"]
impl crate::Resettable for IF0IFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
