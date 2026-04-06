#[doc = "Register `IFS` writer"]
pub type W = crate::W<IfsSpec>;
#[doc = "Field `CH0CD` writer - Set CH0CD Interrupt Flag"]
pub type Ch0cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` writer - Set CH1CD Interrupt Flag"]
pub type Ch1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` writer - Set CH0OF Interrupt Flag"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` writer - Set CH1OF Interrupt Flag"]
pub type Ch1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` writer - Set CH0UF Interrupt Flag"]
pub type Ch0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` writer - Set CH1UF Interrupt Flag"]
pub type Ch1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Set EM23ERR Interrupt Flag"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0APORTCONFLICT` writer - Set OPA0APORTCONFLICT Interrupt Flag"]
pub type Opa0aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1APORTCONFLICT` writer - Set OPA1APORTCONFLICT Interrupt Flag"]
pub type Opa1aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2APORTCONFLICT` writer - Set OPA2APORTCONFLICT Interrupt Flag"]
pub type Opa2aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3APORTCONFLICT` writer - Set OPA3APORTCONFLICT Interrupt Flag"]
pub type Opa3aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - Set OPA0PRSTIMEDERR Interrupt Flag"]
pub type Opa0prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - Set OPA1PRSTIMEDERR Interrupt Flag"]
pub type Opa1prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - Set OPA2PRSTIMEDERR Interrupt Flag"]
pub type Opa2prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3PRSTIMEDERR` writer - Set OPA3PRSTIMEDERR Interrupt Flag"]
pub type Opa3prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0OUTVALID` writer - Set OPA0OUTVALID Interrupt Flag"]
pub type Opa0outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1OUTVALID` writer - Set OPA1OUTVALID Interrupt Flag"]
pub type Opa1outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2OUTVALID` writer - Set OPA2OUTVALID Interrupt Flag"]
pub type Opa2outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3OUTVALID` writer - Set OPA3OUTVALID Interrupt Flag"]
pub type Opa3outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set CH0CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> Ch0cdW<'_, IfsSpec> {
        Ch0cdW::new(self, 0)
    }
    #[doc = "Bit 1 - Set CH1CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> Ch1cdW<'_, IfsSpec> {
        Ch1cdW::new(self, 1)
    }
    #[doc = "Bit 2 - Set CH0OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<'_, IfsSpec> {
        Ch0ofW::new(self, 2)
    }
    #[doc = "Bit 3 - Set CH1OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> Ch1ofW<'_, IfsSpec> {
        Ch1ofW::new(self, 3)
    }
    #[doc = "Bit 4 - Set CH0UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> Ch0ufW<'_, IfsSpec> {
        Ch0ufW::new(self, 4)
    }
    #[doc = "Bit 5 - Set CH1UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> Ch1ufW<'_, IfsSpec> {
        Ch1ufW::new(self, 5)
    }
    #[doc = "Bit 15 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IfsSpec> {
        Em23errW::new(self, 15)
    }
    #[doc = "Bit 16 - Set OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> Opa0aportconflictW<'_, IfsSpec> {
        Opa0aportconflictW::new(self, 16)
    }
    #[doc = "Bit 17 - Set OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> Opa1aportconflictW<'_, IfsSpec> {
        Opa1aportconflictW::new(self, 17)
    }
    #[doc = "Bit 18 - Set OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> Opa2aportconflictW<'_, IfsSpec> {
        Opa2aportconflictW::new(self, 18)
    }
    #[doc = "Bit 19 - Set OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> Opa3aportconflictW<'_, IfsSpec> {
        Opa3aportconflictW::new(self, 19)
    }
    #[doc = "Bit 20 - Set OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> Opa0prstimederrW<'_, IfsSpec> {
        Opa0prstimederrW::new(self, 20)
    }
    #[doc = "Bit 21 - Set OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> Opa1prstimederrW<'_, IfsSpec> {
        Opa1prstimederrW::new(self, 21)
    }
    #[doc = "Bit 22 - Set OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> Opa2prstimederrW<'_, IfsSpec> {
        Opa2prstimederrW::new(self, 22)
    }
    #[doc = "Bit 23 - Set OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> Opa3prstimederrW<'_, IfsSpec> {
        Opa3prstimederrW::new(self, 23)
    }
    #[doc = "Bit 28 - Set OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> Opa0outvalidW<'_, IfsSpec> {
        Opa0outvalidW::new(self, 28)
    }
    #[doc = "Bit 29 - Set OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> Opa1outvalidW<'_, IfsSpec> {
        Opa1outvalidW::new(self, 29)
    }
    #[doc = "Bit 30 - Set OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> Opa2outvalidW<'_, IfsSpec> {
        Opa2outvalidW::new(self, 30)
    }
    #[doc = "Bit 31 - Set OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> Opa3outvalidW<'_, IfsSpec> {
        Opa3outvalidW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfsSpec;
impl crate::RegisterSpec for IfsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IfsSpec {}
