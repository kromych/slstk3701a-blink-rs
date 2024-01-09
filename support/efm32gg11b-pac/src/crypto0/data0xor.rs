#[doc = "Register `DATA0XOR` reader"]
pub type R = crate::R<DATA0XOR_SPEC>;
#[doc = "Register `DATA0XOR` writer"]
pub type W = crate::W<DATA0XOR_SPEC>;
#[doc = "Field `DATA0XOR` reader - XOR Data 0 Access"]
pub type DATA0XOR_R = crate::FieldReader<u32>;
#[doc = "Field `DATA0XOR` writer - XOR Data 0 Access"]
pub type DATA0XOR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    pub fn data0xor(&self) -> DATA0XOR_R {
        DATA0XOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data 0 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0xor(&mut self) -> DATA0XOR_W<DATA0XOR_SPEC> {
        DATA0XOR_W::new(self, 0)
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
#[doc = "DATA0XOR Register Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0xor::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0xor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0XOR_SPEC;
impl crate::RegisterSpec for DATA0XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0xor::R`](R) reader structure"]
impl crate::Readable for DATA0XOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0xor::W`](W) writer structure"]
impl crate::Writable for DATA0XOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0XOR to value 0"]
impl crate::Resettable for DATA0XOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
