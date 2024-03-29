#[doc = "Register `ETMTRIGGER` reader"]
pub type R = crate::R<ETMTRIGGER_SPEC>;
#[doc = "Register `ETMTRIGGER` writer"]
pub type W = crate::W<ETMTRIGGER_SPEC>;
#[doc = "Field `RESA` reader - ETM Resource A"]
pub type RESA_R = crate::FieldReader;
#[doc = "Field `RESA` writer - ETM Resource A"]
pub type RESA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RESB` reader - ETM Resource B"]
pub type RESB_R = crate::FieldReader;
#[doc = "Field `RESB` writer - ETM Resource B"]
pub type RESB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ETMFCN` reader - ETM Function"]
pub type ETMFCN_R = crate::FieldReader;
#[doc = "Field `ETMFCN` writer - ETM Function"]
pub type ETMFCN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    pub fn resa(&self) -> RESA_R {
        RESA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    pub fn resb(&self) -> RESB_R {
        RESB_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    pub fn etmfcn(&self) -> ETMFCN_R {
        ETMFCN_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ETM Resource A"]
    #[inline(always)]
    #[must_use]
    pub fn resa(&mut self) -> RESA_W<ETMTRIGGER_SPEC> {
        RESA_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - ETM Resource B"]
    #[inline(always)]
    #[must_use]
    pub fn resb(&mut self) -> RESB_W<ETMTRIGGER_SPEC> {
        RESB_W::new(self, 7)
    }
    #[doc = "Bits 14:16 - ETM Function"]
    #[inline(always)]
    #[must_use]
    pub fn etmfcn(&mut self) -> ETMFCN_W<ETMTRIGGER_SPEC> {
        ETMFCN_W::new(self, 14)
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
#[doc = "ETM Trigger Event Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etmtrigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etmtrigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETMTRIGGER_SPEC;
impl crate::RegisterSpec for ETMTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmtrigger::R`](R) reader structure"]
impl crate::Readable for ETMTRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etmtrigger::W`](W) writer structure"]
impl crate::Writable for ETMTRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETMTRIGGER to value 0"]
impl crate::Resettable for ETMTRIGGER_SPEC {
    const RESET_VALUE: u32 = 0;
}
