#[doc = "Register `DCDCLNCOMPCTRL` reader"]
pub type R = crate::R<DCDCLNCOMPCTRL_SPEC>;
#[doc = "Register `DCDCLNCOMPCTRL` writer"]
pub type W = crate::W<DCDCLNCOMPCTRL_SPEC>;
#[doc = "Field `COMPENR1` reader - Low Noise Mode Compensator R1 Trim Value"]
pub type COMPENR1_R = crate::FieldReader;
#[doc = "Field `COMPENR1` writer - Low Noise Mode Compensator R1 Trim Value"]
pub type COMPENR1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPENR2` reader - Low Noise Mode Compensator R2 Trim Value"]
pub type COMPENR2_R = crate::FieldReader;
#[doc = "Field `COMPENR2` writer - Low Noise Mode Compensator R2 Trim Value"]
pub type COMPENR2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `COMPENR3` reader - Low Noise Mode Compensator R3 Trim Value"]
pub type COMPENR3_R = crate::FieldReader;
#[doc = "Field `COMPENR3` writer - Low Noise Mode Compensator R3 Trim Value"]
pub type COMPENR3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COMPENC1` reader - Low Noise Mode Compensator C1 Trim Value"]
pub type COMPENC1_R = crate::FieldReader;
#[doc = "Field `COMPENC1` writer - Low Noise Mode Compensator C1 Trim Value"]
pub type COMPENC1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPENC2` reader - Low Noise Mode Compensator C2 Trim Value"]
pub type COMPENC2_R = crate::FieldReader;
#[doc = "Field `COMPENC2` writer - Low Noise Mode Compensator C2 Trim Value"]
pub type COMPENC2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPENC3` reader - Low Noise Mode Compensator C3 Trim Value"]
pub type COMPENC3_R = crate::FieldReader;
#[doc = "Field `COMPENC3` writer - Low Noise Mode Compensator C3 Trim Value"]
pub type COMPENC3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&self) -> COMPENR1_R {
        COMPENR1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&self) -> COMPENR2_R {
        COMPENR2_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&self) -> COMPENR3_R {
        COMPENR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&self) -> COMPENC1_R {
        COMPENC1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&self) -> COMPENC2_R {
        COMPENC2_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&self) -> COMPENC3_R {
        COMPENC3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr1(&mut self) -> COMPENR1_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENR1_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr2(&mut self) -> COMPENR2_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENR2_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr3(&mut self) -> COMPENR3_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENR3_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc1(&mut self) -> COMPENC1_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENC1_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc2(&mut self) -> COMPENC2_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENC2_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc3(&mut self) -> COMPENC3_W<DCDCLNCOMPCTRL_SPEC> {
        COMPENC3_W::new(self, 28)
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
#[doc = "DCDC Low Noise Compensator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdclncompctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdclncompctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCLNCOMPCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNCOMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclncompctrl::R`](R) reader structure"]
impl crate::Readable for DCDCLNCOMPCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdclncompctrl::W`](W) writer structure"]
impl crate::Writable for DCDCLNCOMPCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCDCLNCOMPCTRL to value 0x5720_4077"]
impl crate::Resettable for DCDCLNCOMPCTRL_SPEC {
    const RESET_VALUE: u32 = 0x5720_4077;
}
