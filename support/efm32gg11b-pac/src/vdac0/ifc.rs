#[doc = "Register `IFC` writer"]
pub type W = crate::W<IfcSpec>;
#[doc = "Field `CH0CD` writer - Clear CH0CD Interrupt Flag"]
pub type Ch0cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` writer - Clear CH1CD Interrupt Flag"]
pub type Ch1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` writer - Clear CH0OF Interrupt Flag"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` writer - Clear CH1OF Interrupt Flag"]
pub type Ch1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` writer - Clear CH0UF Interrupt Flag"]
pub type Ch0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` writer - Clear CH1UF Interrupt Flag"]
pub type Ch1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` writer - Clear EM23ERR Interrupt Flag"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0APORTCONFLICT` writer - Clear OPA0APORTCONFLICT Interrupt Flag"]
pub type Opa0aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1APORTCONFLICT` writer - Clear OPA1APORTCONFLICT Interrupt Flag"]
pub type Opa1aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2APORTCONFLICT` writer - Clear OPA2APORTCONFLICT Interrupt Flag"]
pub type Opa2aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3APORTCONFLICT` writer - Clear OPA3APORTCONFLICT Interrupt Flag"]
pub type Opa3aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - Clear OPA0PRSTIMEDERR Interrupt Flag"]
pub type Opa0prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - Clear OPA1PRSTIMEDERR Interrupt Flag"]
pub type Opa1prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - Clear OPA2PRSTIMEDERR Interrupt Flag"]
pub type Opa2prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3PRSTIMEDERR` writer - Clear OPA3PRSTIMEDERR Interrupt Flag"]
pub type Opa3prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0OUTVALID` writer - Clear OPA0OUTVALID Interrupt Flag"]
pub type Opa0outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1OUTVALID` writer - Clear OPA1OUTVALID Interrupt Flag"]
pub type Opa1outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2OUTVALID` writer - Clear OPA2OUTVALID Interrupt Flag"]
pub type Opa2outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3OUTVALID` writer - Clear OPA3OUTVALID Interrupt Flag"]
pub type Opa3outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear CH0CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> Ch0cdW<'_, IfcSpec> {
        Ch0cdW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CH1CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> Ch1cdW<'_, IfcSpec> {
        Ch1cdW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear CH0OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<'_, IfcSpec> {
        Ch0ofW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear CH1OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> Ch1ofW<'_, IfcSpec> {
        Ch1ofW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear CH0UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> Ch0ufW<'_, IfcSpec> {
        Ch0ufW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CH1UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> Ch1ufW<'_, IfcSpec> {
        Ch1ufW::new(self, 5)
    }
    #[doc = "Bit 15 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IfcSpec> {
        Em23errW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> Opa0aportconflictW<'_, IfcSpec> {
        Opa0aportconflictW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> Opa1aportconflictW<'_, IfcSpec> {
        Opa1aportconflictW::new(self, 17)
    }
    #[doc = "Bit 18 - Clear OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> Opa2aportconflictW<'_, IfcSpec> {
        Opa2aportconflictW::new(self, 18)
    }
    #[doc = "Bit 19 - Clear OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> Opa3aportconflictW<'_, IfcSpec> {
        Opa3aportconflictW::new(self, 19)
    }
    #[doc = "Bit 20 - Clear OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> Opa0prstimederrW<'_, IfcSpec> {
        Opa0prstimederrW::new(self, 20)
    }
    #[doc = "Bit 21 - Clear OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> Opa1prstimederrW<'_, IfcSpec> {
        Opa1prstimederrW::new(self, 21)
    }
    #[doc = "Bit 22 - Clear OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> Opa2prstimederrW<'_, IfcSpec> {
        Opa2prstimederrW::new(self, 22)
    }
    #[doc = "Bit 23 - Clear OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> Opa3prstimederrW<'_, IfcSpec> {
        Opa3prstimederrW::new(self, 23)
    }
    #[doc = "Bit 28 - Clear OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> Opa0outvalidW<'_, IfcSpec> {
        Opa0outvalidW::new(self, 28)
    }
    #[doc = "Bit 29 - Clear OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> Opa1outvalidW<'_, IfcSpec> {
        Opa1outvalidW::new(self, 29)
    }
    #[doc = "Bit 30 - Clear OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> Opa2outvalidW<'_, IfcSpec> {
        Opa2outvalidW::new(self, 30)
    }
    #[doc = "Bit 31 - Clear OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> Opa3outvalidW<'_, IfcSpec> {
        Opa3outvalidW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcSpec;
impl crate::RegisterSpec for IfcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IfcSpec {}
