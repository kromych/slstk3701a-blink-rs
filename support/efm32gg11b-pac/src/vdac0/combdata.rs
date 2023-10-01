#[doc = "Register `COMBDATA` reader"]
pub type R = crate::R<COMBDATA_SPEC>;
#[doc = "Register `COMBDATA` writer"]
pub type W = crate::W<COMBDATA_SPEC>;
#[doc = "Field `CH0DATA` reader - Channel 0 Data"]
pub type CH0DATA_R = crate::FieldReader<u16>;
#[doc = "Field `CH0DATA` writer - Channel 0 Data"]
pub type CH0DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `CH1DATA` reader - Channel 1 Data"]
pub type CH1DATA_R = crate::FieldReader<u16>;
#[doc = "Field `CH1DATA` writer - Channel 1 Data"]
pub type CH1DATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&self) -> CH0DATA_R {
        CH0DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&self) -> CH1DATA_R {
        CH1DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    #[must_use]
    pub fn ch0data(&mut self) -> CH0DATA_W<COMBDATA_SPEC, 0> {
        CH0DATA_W::new(self)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    #[must_use]
    pub fn ch1data(&mut self) -> CH1DATA_W<COMBDATA_SPEC, 16> {
        CH1DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Combined Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`combdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMBDATA_SPEC;
impl crate::RegisterSpec for COMBDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`combdata::R`](R) reader structure"]
impl crate::Readable for COMBDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`combdata::W`](W) writer structure"]
impl crate::Writable for COMBDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMBDATA to value 0x0800_0800"]
impl crate::Resettable for COMBDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0800;
}
