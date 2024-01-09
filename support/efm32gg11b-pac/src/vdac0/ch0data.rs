#[doc = "Register `CH0DATA` reader"]
pub type R = crate::R<CH0DATA_SPEC>;
#[doc = "Register `CH0DATA` writer"]
pub type W = crate::W<CH0DATA_SPEC>;
#[doc = "Field `DATA` reader - Channel 0 Data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Channel 0 Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<CH0DATA_SPEC> {
        DATA_W::new(self, 0)
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
#[doc = "Channel 0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0DATA_SPEC;
impl crate::RegisterSpec for CH0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0data::R`](R) reader structure"]
impl crate::Readable for CH0DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch0data::W`](W) writer structure"]
impl crate::Writable for CH0DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0DATA to value 0x0800"]
impl crate::Resettable for CH0DATA_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
