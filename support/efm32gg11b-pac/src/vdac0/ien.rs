#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CH0CD` reader - CH0CD Interrupt Enable"]
pub type Ch0cdR = crate::BitReader;
#[doc = "Field `CH0CD` writer - CH0CD Interrupt Enable"]
pub type Ch0cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` reader - CH1CD Interrupt Enable"]
pub type Ch1cdR = crate::BitReader;
#[doc = "Field `CH1CD` writer - CH1CD Interrupt Enable"]
pub type Ch1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` reader - CH0OF Interrupt Enable"]
pub type Ch0ofR = crate::BitReader;
#[doc = "Field `CH0OF` writer - CH0OF Interrupt Enable"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` reader - CH1OF Interrupt Enable"]
pub type Ch1ofR = crate::BitReader;
#[doc = "Field `CH1OF` writer - CH1OF Interrupt Enable"]
pub type Ch1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` reader - CH0UF Interrupt Enable"]
pub type Ch0ufR = crate::BitReader;
#[doc = "Field `CH0UF` writer - CH0UF Interrupt Enable"]
pub type Ch0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` reader - CH1UF Interrupt Enable"]
pub type Ch1ufR = crate::BitReader;
#[doc = "Field `CH1UF` writer - CH1UF Interrupt Enable"]
pub type Ch1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0BL` reader - CH0BL Interrupt Enable"]
pub type Ch0blR = crate::BitReader;
#[doc = "Field `CH0BL` writer - CH0BL Interrupt Enable"]
pub type Ch0blW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1BL` reader - CH1BL Interrupt Enable"]
pub type Ch1blR = crate::BitReader;
#[doc = "Field `CH1BL` writer - CH1BL Interrupt Enable"]
pub type Ch1blW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type Em23errR = crate::BitReader;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type Em23errW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0APORTCONFLICT Interrupt Enable"]
pub type Opa0aportconflictR = crate::BitReader;
#[doc = "Field `OPA0APORTCONFLICT` writer - OPA0APORTCONFLICT Interrupt Enable"]
pub type Opa0aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1APORTCONFLICT Interrupt Enable"]
pub type Opa1aportconflictR = crate::BitReader;
#[doc = "Field `OPA1APORTCONFLICT` writer - OPA1APORTCONFLICT Interrupt Enable"]
pub type Opa1aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2APORTCONFLICT Interrupt Enable"]
pub type Opa2aportconflictR = crate::BitReader;
#[doc = "Field `OPA2APORTCONFLICT` writer - OPA2APORTCONFLICT Interrupt Enable"]
pub type Opa2aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3APORTCONFLICT Interrupt Enable"]
pub type Opa3aportconflictR = crate::BitReader;
#[doc = "Field `OPA3APORTCONFLICT` writer - OPA3APORTCONFLICT Interrupt Enable"]
pub type Opa3aportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0PRSTIMEDERR` reader - OPA0PRSTIMEDERR Interrupt Enable"]
pub type Opa0prstimederrR = crate::BitReader;
#[doc = "Field `OPA0PRSTIMEDERR` writer - OPA0PRSTIMEDERR Interrupt Enable"]
pub type Opa0prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1PRSTIMEDERR` reader - OPA1PRSTIMEDERR Interrupt Enable"]
pub type Opa1prstimederrR = crate::BitReader;
#[doc = "Field `OPA1PRSTIMEDERR` writer - OPA1PRSTIMEDERR Interrupt Enable"]
pub type Opa1prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2PRSTIMEDERR` reader - OPA2PRSTIMEDERR Interrupt Enable"]
pub type Opa2prstimederrR = crate::BitReader;
#[doc = "Field `OPA2PRSTIMEDERR` writer - OPA2PRSTIMEDERR Interrupt Enable"]
pub type Opa2prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3PRSTIMEDERR` reader - OPA3PRSTIMEDERR Interrupt Enable"]
pub type Opa3prstimederrR = crate::BitReader;
#[doc = "Field `OPA3PRSTIMEDERR` writer - OPA3PRSTIMEDERR Interrupt Enable"]
pub type Opa3prstimederrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA0OUTVALID` reader - OPA0OUTVALID Interrupt Enable"]
pub type Opa0outvalidR = crate::BitReader;
#[doc = "Field `OPA0OUTVALID` writer - OPA0OUTVALID Interrupt Enable"]
pub type Opa0outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA1OUTVALID` reader - OPA1OUTVALID Interrupt Enable"]
pub type Opa1outvalidR = crate::BitReader;
#[doc = "Field `OPA1OUTVALID` writer - OPA1OUTVALID Interrupt Enable"]
pub type Opa1outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA2OUTVALID` reader - OPA2OUTVALID Interrupt Enable"]
pub type Opa2outvalidR = crate::BitReader;
#[doc = "Field `OPA2OUTVALID` writer - OPA2OUTVALID Interrupt Enable"]
pub type Opa2outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPA3OUTVALID` reader - OPA3OUTVALID Interrupt Enable"]
pub type Opa3outvalidR = crate::BitReader;
#[doc = "Field `OPA3OUTVALID` writer - OPA3OUTVALID Interrupt Enable"]
pub type Opa3outvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&self) -> Ch0cdR {
        Ch0cdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&self) -> Ch1cdR {
        Ch1cdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&self) -> Ch1ofR {
        Ch1ofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&self) -> Ch0ufR {
        Ch0ufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&self) -> Ch1ufR {
        Ch1ufR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&self) -> Ch0blR {
        Ch0blR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&self) -> Ch1blR {
        Ch1blR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> Em23errR {
        Em23errR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> Opa0aportconflictR {
        Opa0aportconflictR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> Opa1aportconflictR {
        Opa1aportconflictR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> Opa2aportconflictR {
        Opa2aportconflictR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> Opa3aportconflictR {
        Opa3aportconflictR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> Opa0prstimederrR {
        Opa0prstimederrR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> Opa1prstimederrR {
        Opa1prstimederrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> Opa2prstimederrR {
        Opa2prstimederrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> Opa3prstimederrR {
        Opa3prstimederrR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> Opa0outvalidR {
        Opa0outvalidR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> Opa1outvalidR {
        Opa1outvalidR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> Opa2outvalidR {
        Opa2outvalidR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> Opa3outvalidR {
        Opa3outvalidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> Ch0cdW<'_, IenSpec> {
        Ch0cdW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1CD Interrupt Enable"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> Ch1cdW<'_, IenSpec> {
        Ch1cdW::new(self, 1)
    }
    #[doc = "Bit 2 - CH0OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<'_, IenSpec> {
        Ch0ofW::new(self, 2)
    }
    #[doc = "Bit 3 - CH1OF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> Ch1ofW<'_, IenSpec> {
        Ch1ofW::new(self, 3)
    }
    #[doc = "Bit 4 - CH0UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> Ch0ufW<'_, IenSpec> {
        Ch0ufW::new(self, 4)
    }
    #[doc = "Bit 5 - CH1UF Interrupt Enable"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> Ch1ufW<'_, IenSpec> {
        Ch1ufW::new(self, 5)
    }
    #[doc = "Bit 6 - CH0BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch0bl(&mut self) -> Ch0blW<'_, IenSpec> {
        Ch0blW::new(self, 6)
    }
    #[doc = "Bit 7 - CH1BL Interrupt Enable"]
    #[inline(always)]
    pub fn ch1bl(&mut self) -> Ch1blW<'_, IenSpec> {
        Ch1blW::new(self, 7)
    }
    #[doc = "Bit 15 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&mut self) -> Em23errW<'_, IenSpec> {
        Em23errW::new(self, 15)
    }
    #[doc = "Bit 16 - OPA0APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> Opa0aportconflictW<'_, IenSpec> {
        Opa0aportconflictW::new(self, 16)
    }
    #[doc = "Bit 17 - OPA1APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> Opa1aportconflictW<'_, IenSpec> {
        Opa1aportconflictW::new(self, 17)
    }
    #[doc = "Bit 18 - OPA2APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> Opa2aportconflictW<'_, IenSpec> {
        Opa2aportconflictW::new(self, 18)
    }
    #[doc = "Bit 19 - OPA3APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> Opa3aportconflictW<'_, IenSpec> {
        Opa3aportconflictW::new(self, 19)
    }
    #[doc = "Bit 20 - OPA0PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> Opa0prstimederrW<'_, IenSpec> {
        Opa0prstimederrW::new(self, 20)
    }
    #[doc = "Bit 21 - OPA1PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> Opa1prstimederrW<'_, IenSpec> {
        Opa1prstimederrW::new(self, 21)
    }
    #[doc = "Bit 22 - OPA2PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> Opa2prstimederrW<'_, IenSpec> {
        Opa2prstimederrW::new(self, 22)
    }
    #[doc = "Bit 23 - OPA3PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> Opa3prstimederrW<'_, IenSpec> {
        Opa3prstimederrW::new(self, 23)
    }
    #[doc = "Bit 28 - OPA0OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> Opa0outvalidW<'_, IenSpec> {
        Opa0outvalidW::new(self, 28)
    }
    #[doc = "Bit 29 - OPA1OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> Opa1outvalidW<'_, IenSpec> {
        Opa1outvalidW::new(self, 29)
    }
    #[doc = "Bit 30 - OPA2OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> Opa2outvalidW<'_, IenSpec> {
        Opa2outvalidW::new(self, 30)
    }
    #[doc = "Bit 31 - OPA3OUTVALID Interrupt Enable"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> Opa3outvalidW<'_, IenSpec> {
        Opa3outvalidW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
