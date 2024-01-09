#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `CH0CD` writer - Set CH0CD Interrupt Flag"]
pub type CH0CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` writer - Set CH1CD Interrupt Flag"]
pub type CH1CD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` writer - Set CH0OF Interrupt Flag"]
pub type CH0OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` writer - Set CH1OF Interrupt Flag"]
pub type CH1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` writer - Set CH0UF Interrupt Flag"]
pub type CH0UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` writer - Set CH1UF Interrupt Flag"]
pub type CH1UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Set EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0APORTCONFLICT` writer - Set OPA0APORTCONFLICT Interrupt Flag"]
pub type OPA0APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1APORTCONFLICT` writer - Set OPA1APORTCONFLICT Interrupt Flag"]
pub type OPA1APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2APORTCONFLICT` writer - Set OPA2APORTCONFLICT Interrupt Flag"]
pub type OPA2APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3APORTCONFLICT` writer - Set OPA3APORTCONFLICT Interrupt Flag"]
pub type OPA3APORTCONFLICT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - Set OPA0PRSTIMEDERR Interrupt Flag"]
pub type OPA0PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - Set OPA1PRSTIMEDERR Interrupt Flag"]
pub type OPA1PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - Set OPA2PRSTIMEDERR Interrupt Flag"]
pub type OPA2PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3PRSTIMEDERR` writer - Set OPA3PRSTIMEDERR Interrupt Flag"]
pub type OPA3PRSTIMEDERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0OUTVALID` writer - Set OPA0OUTVALID Interrupt Flag"]
pub type OPA0OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1OUTVALID` writer - Set OPA1OUTVALID Interrupt Flag"]
pub type OPA1OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2OUTVALID` writer - Set OPA2OUTVALID Interrupt Flag"]
pub type OPA2OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3OUTVALID` writer - Set OPA3OUTVALID Interrupt Flag"]
pub type OPA3OUTVALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CH0CD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cd(&mut self) -> CH0CD_W<IFS_SPEC> {
        CH0CD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set CH1CD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cd(&mut self) -> CH1CD_W<IFS_SPEC> {
        CH1CD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set CH0OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<IFS_SPEC> {
        CH0OF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set CH1OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<IFS_SPEC> {
        CH1OF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set CH0UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0uf(&mut self) -> CH0UF_W<IFS_SPEC> {
        CH0UF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set CH1UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1uf(&mut self) -> CH1UF_W<IFS_SPEC> {
        CH1UF_W::new(self, 5)
    }
    #[doc = "Bit 15 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<IFS_SPEC> {
        EM23ERR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W<IFS_SPEC> {
        OPA0APORTCONFLICT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W<IFS_SPEC> {
        OPA1APORTCONFLICT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W<IFS_SPEC> {
        OPA2APORTCONFLICT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W<IFS_SPEC> {
        OPA3APORTCONFLICT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W<IFS_SPEC> {
        OPA0PRSTIMEDERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W<IFS_SPEC> {
        OPA1PRSTIMEDERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W<IFS_SPEC> {
        OPA2PRSTIMEDERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W<IFS_SPEC> {
        OPA3PRSTIMEDERR_W::new(self, 23)
    }
    #[doc = "Bit 28 - Set OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W<IFS_SPEC> {
        OPA0OUTVALID_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W<IFS_SPEC> {
        OPA1OUTVALID_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W<IFS_SPEC> {
        OPA2OUTVALID_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W<IFS_SPEC> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}
