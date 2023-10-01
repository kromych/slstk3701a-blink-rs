#[doc = "Register `IF0IEN` reader"]
pub type R = crate::R<IF0IEN_SPEC>;
#[doc = "Register `IF0IEN` writer"]
pub type W = crate::W<IF0IEN_SPEC>;
#[doc = "Field `MESSAGE` reader - MESSAGE Interrupt Enable"]
pub type MESSAGE_R = crate::FieldReader<u32>;
#[doc = "Field `MESSAGE` writer - MESSAGE Interrupt Enable"]
pub type MESSAGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn message(&mut self) -> MESSAGE_W<IF0IEN_SPEC, 0> {
        MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Message Object Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if0ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if0ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF0IEN_SPEC;
impl crate::RegisterSpec for IF0IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if0ien::R`](R) reader structure"]
impl crate::Readable for IF0IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`if0ien::W`](W) writer structure"]
impl crate::Writable for IF0IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF0IEN to value 0xffff_ffff"]
impl crate::Resettable for IF0IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
