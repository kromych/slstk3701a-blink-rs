#[doc = "Register `SCANRES` reader"]
pub type R = crate::R<SCANRES_SPEC>;
#[doc = "Register `SCANRES` writer"]
pub type W = crate::W<SCANRES_SPEC>;
#[doc = "Field `SCANRES` reader - Scan Results"]
pub type SCANRES_R = crate::FieldReader<u16>;
#[doc = "Field `SCANRES` writer - Scan Results"]
pub type SCANRES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `STEPDIR` reader - Direction of Previous Step Detection"]
pub type STEPDIR_R = crate::FieldReader<u16>;
#[doc = "Field `STEPDIR` writer - Direction of Previous Step Detection"]
pub type STEPDIR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&self) -> SCANRES_R {
        SCANRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&self) -> STEPDIR_R {
        STEPDIR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    #[must_use]
    pub fn scanres(&mut self) -> SCANRES_W<SCANRES_SPEC, 0> {
        SCANRES_W::new(self)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    #[must_use]
    pub fn stepdir(&mut self) -> STEPDIR_W<SCANRES_SPEC, 16> {
        STEPDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scan Result Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANRES_SPEC;
impl crate::RegisterSpec for SCANRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanres::R`](R) reader structure"]
impl crate::Readable for SCANRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scanres::W`](W) writer structure"]
impl crate::Writable for SCANRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANRES to value 0"]
impl crate::Resettable for SCANRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
