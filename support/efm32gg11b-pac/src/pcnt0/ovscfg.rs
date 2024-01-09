#[doc = "Register `OVSCFG` reader"]
pub type R = crate::R<OVSCFG_SPEC>;
#[doc = "Register `OVSCFG` writer"]
pub type W = crate::W<OVSCFG_SPEC>;
#[doc = "Field `FILTLEN` reader - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FILTLEN_R = crate::FieldReader;
#[doc = "Field `FILTLEN` writer - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FILTLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLUTTERRM` reader - Flutter Remove"]
pub type FLUTTERRM_R = crate::BitReader;
#[doc = "Field `FLUTTERRM` writer - Flutter Remove"]
pub type FLUTTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FLUTTERRM_R {
        FLUTTERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    #[must_use]
    pub fn filtlen(&mut self) -> FILTLEN_W<OVSCFG_SPEC> {
        FILTLEN_W::new(self, 0)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    #[must_use]
    pub fn flutterrm(&mut self) -> FLUTTERRM_W<OVSCFG_SPEC> {
        FLUTTERRM_W::new(self, 12)
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
#[doc = "Oversampling Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OVSCFG_SPEC;
impl crate::RegisterSpec for OVSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovscfg::R`](R) reader structure"]
impl crate::Readable for OVSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ovscfg::W`](W) writer structure"]
impl crate::Writable for OVSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OVSCFG to value 0"]
impl crate::Resettable for OVSCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
