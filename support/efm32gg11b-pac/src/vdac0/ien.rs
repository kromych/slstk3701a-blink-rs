#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `CH0CD` reader - CH0CD Interrupt Enable"]
pub type CH0CD_R = crate::BitReader;
#[doc = "Field `CH0CD` writer - CH0CD Interrupt Enable"]
pub type CH0CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` reader - CH1CD Interrupt Enable"]
pub type CH1CD_R = crate::BitReader;
#[doc = "Field `CH1CD` writer - CH1CD Interrupt Enable"]
pub type CH1CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` reader - CH0OF Interrupt Enable"]
pub type CH0OF_R = crate::BitReader;
#[doc = "Field `CH0OF` writer - CH0OF Interrupt Enable"]
pub type CH0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` reader - CH1OF Interrupt Enable"]
pub type CH1OF_R = crate::BitReader;
#[doc = "Field `CH1OF` writer - CH1OF Interrupt Enable"]
pub type CH1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` reader - CH0UF Interrupt Enable"]
pub type CH0UF_R = crate::BitReader;
#[doc = "Field `CH0UF` writer - CH0UF Interrupt Enable"]
pub type CH0UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` reader - CH1UF Interrupt Enable"]
pub type CH1UF_R = crate::BitReader;
#[doc = "Field `CH1UF` writer - CH1UF Interrupt Enable"]
pub type CH1UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0BL` reader - CH0BL Interrupt Enable"]
pub type CH0BL_R = crate::BitReader;
#[doc = "Field `CH0BL` writer - CH0BL Interrupt Enable"]
pub type CH0BL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1BL` reader - CH1BL Interrupt Enable"]
pub type CH1BL_R = crate::BitReader;
#[doc = "Field `CH1BL` writer - CH1BL Interrupt Enable"]
pub type CH1BL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type EM23ERR_R = crate::BitReader;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type EM23ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0APORTCONFLICT Interrupt Enable"]
pub type OPA0APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `OPA0APORTCONFLICT` writer - OPA0APORTCONFLICT Interrupt Enable"]
pub type OPA0APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1APORTCONFLICT Interrupt Enable"]
pub type OPA1APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `OPA1APORTCONFLICT` writer - OPA1APORTCONFLICT Interrupt Enable"]
pub type OPA1APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2APORTCONFLICT Interrupt Enable"]
pub type OPA2APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `OPA2APORTCONFLICT` writer - OPA2APORTCONFLICT Interrupt Enable"]
pub type OPA2APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3APORTCONFLICT Interrupt Enable"]
pub type OPA3APORTCONFLICT_R = crate::BitReader;
#[doc = "Field `OPA3APORTCONFLICT` writer - OPA3APORTCONFLICT Interrupt Enable"]
pub type OPA3APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0PRSTIMEDERR` reader - OPA0PRSTIMEDERR Interrupt Enable"]
pub type OPA0PRSTIMEDERR_R = crate::BitReader;
#[doc = "Field `OPA0PRSTIMEDERR` writer - OPA0PRSTIMEDERR Interrupt Enable"]
pub type OPA0PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1PRSTIMEDERR` reader - OPA1PRSTIMEDERR Interrupt Enable"]
pub type OPA1PRSTIMEDERR_R = crate::BitReader;
#[doc = "Field `OPA1PRSTIMEDERR` writer - OPA1PRSTIMEDERR Interrupt Enable"]
pub type OPA1PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2PRSTIMEDERR` reader - OPA2PRSTIMEDERR Interrupt Enable"]
pub type OPA2PRSTIMEDERR_R = crate::BitReader;
#[doc = "Field `OPA2PRSTIMEDERR` writer - OPA2PRSTIMEDERR Interrupt Enable"]
pub type OPA2PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3PRSTIMEDERR` reader - OPA3PRSTIMEDERR Interrupt Enable"]
pub type OPA3PRSTIMEDERR_R = crate::BitReader;
#[doc = "Field `OPA3PRSTIMEDERR` writer - OPA3PRSTIMEDERR Interrupt Enable"]
pub type OPA3PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0OUTVALID` reader - OPA0OUTVALID Interrupt Enable"]
pub type OPA0OUTVALID_R = crate::BitReader;
#[doc = "Field `OPA0OUTVALID` writer - OPA0OUTVALID Interrupt Enable"]
pub type OPA0OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1OUTVALID` reader - OPA1OUTVALID Interrupt Enable"]
pub type OPA1OUTVALID_R = crate::BitReader;
#[doc = "Field `OPA1OUTVALID` writer - OPA1OUTVALID Interrupt Enable"]
pub type OPA1OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2OUTVALID` reader - OPA2OUTVALID Interrupt Enable"]
pub type OPA2OUTVALID_R = crate::BitReader;
#[doc = "Field `OPA2OUTVALID` writer - OPA2OUTVALID Interrupt Enable"]
pub type OPA2OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3OUTVALID` reader - OPA3OUTVALID Interrupt Enable"]
pub type OPA3OUTVALID_R = crate::BitReader;
#[doc = "Field `OPA3OUTVALID` writer - OPA3OUTVALID Interrupt Enable"]
pub type OPA3OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> OPA0PRSTIMEDERR_R {
        OPA0PRSTIMEDERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> OPA1PRSTIMEDERR_R {
        OPA1PRSTIMEDERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> OPA2PRSTIMEDERR_R {
        OPA2PRSTIMEDERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> OPA3PRSTIMEDERR_R {
        OPA3PRSTIMEDERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cd(&mut self) -> CH0CD_W<IEN_SPEC> {
        CH0CD_W::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cd(&mut self) -> CH1CD_W<IEN_SPEC> {
        CH1CD_W::new(self, 1)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<IEN_SPEC> {
        CH0OF_W::new(self, 2)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<IEN_SPEC> {
        CH1OF_W::new(self, 3)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0uf(&mut self) -> CH0UF_W<IEN_SPEC> {
        CH0UF_W::new(self, 4)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1uf(&mut self) -> CH1UF_W<IEN_SPEC> {
        CH1UF_W::new(self, 5)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0bl(&mut self) -> CH0BL_W<IEN_SPEC> {
        CH0BL_W::new(self, 6)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1bl(&mut self) -> CH1BL_W<IEN_SPEC> {
        CH1BL_W::new(self, 7)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<IEN_SPEC> {
        EM23ERR_W::new(self, 15)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W<IEN_SPEC> {
        OPA0APORTCONFLICT_W::new(self, 16)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W<IEN_SPEC> {
        OPA1APORTCONFLICT_W::new(self, 17)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W<IEN_SPEC> {
        OPA2APORTCONFLICT_W::new(self, 18)
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W<IEN_SPEC> {
        OPA3APORTCONFLICT_W::new(self, 19)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W<IEN_SPEC> {
        OPA0PRSTIMEDERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W<IEN_SPEC> {
        OPA1PRSTIMEDERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W<IEN_SPEC> {
        OPA2PRSTIMEDERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W<IEN_SPEC> {
        OPA3PRSTIMEDERR_W::new(self, 23)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W<IEN_SPEC> {
        OPA0OUTVALID_W::new(self, 28)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W<IEN_SPEC> {
        OPA1OUTVALID_W::new(self, 29)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W<IEN_SPEC> {
        OPA2OUTVALID_W::new(self, 30)
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W<IEN_SPEC> {
        OPA3OUTVALID_W::new(self, 31)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
