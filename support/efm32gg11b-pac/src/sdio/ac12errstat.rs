#[doc = "Register `AC12ERRSTAT` reader"]
pub type R = crate::R<Ac12errstatSpec>;
#[doc = "Register `AC12ERRSTAT` writer"]
pub type W = crate::W<Ac12errstatSpec>;
#[doc = "Field `AC12NOTEXE` reader - Auto CMD12 Not Executed"]
pub type Ac12notexeR = crate::BitReader;
#[doc = "Field `AC12TOE` reader - Auto CMD12 Timeout Error"]
pub type Ac12toeR = crate::BitReader;
#[doc = "Field `AC12CRCERR` reader - Auto CMD CRC Error"]
pub type Ac12crcerrR = crate::BitReader;
#[doc = "Field `AC12ENDBITERR` reader - Auto CMD End Bit Error"]
pub type Ac12endbiterrR = crate::BitReader;
#[doc = "Field `AC12INDEXERR` reader - Auto CMD Index Error"]
pub type Ac12indexerrR = crate::BitReader;
#[doc = "Field `CNIBAC12ERR` reader - Command Not Issued By Auto CMD12 Error"]
pub type Cnibac12errR = crate::BitReader;
#[doc = "UHS Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Uhsmodesel {
    #[doc = "0: SDR12"]
    Sdr12 = 0,
    #[doc = "1: SDR25"]
    Sdr25 = 1,
    #[doc = "2: SDR50"]
    Sdr50 = 2,
    #[doc = "3: SDR104"]
    Sdr104 = 3,
    #[doc = "4: DDR50"]
    Ddr50 = 4,
}
impl From<Uhsmodesel> for u8 {
    #[inline(always)]
    fn from(variant: Uhsmodesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uhsmodesel {
    type Ux = u8;
}
impl crate::IsEnum for Uhsmodesel {}
#[doc = "Field `UHSMODESEL` reader - UHS Mode Select"]
pub type UhsmodeselR = crate::FieldReader<Uhsmodesel>;
impl UhsmodeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uhsmodesel> {
        match self.bits {
            0 => Some(Uhsmodesel::Sdr12),
            1 => Some(Uhsmodesel::Sdr25),
            2 => Some(Uhsmodesel::Sdr50),
            3 => Some(Uhsmodesel::Sdr104),
            4 => Some(Uhsmodesel::Ddr50),
            _ => None,
        }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == Uhsmodesel::Sdr12
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == Uhsmodesel::Sdr25
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == Uhsmodesel::Sdr50
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == Uhsmodesel::Sdr104
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == Uhsmodesel::Ddr50
    }
}
#[doc = "Field `UHSMODESEL` writer - UHS Mode Select"]
pub type UhsmodeselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Uhsmodesel>;
impl<'a, REG> UhsmodeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodesel::Sdr12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodesel::Sdr25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodesel::Sdr50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodesel::Sdr104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut crate::W<REG> {
        self.variant(Uhsmodesel::Ddr50)
    }
}
#[doc = "Field `SIGEN1P8V` reader - Voltage 1.8V Signal Enable"]
pub type Sigen1p8vR = crate::BitReader;
#[doc = "Field `SIGEN1P8V` writer - Voltage 1.8V Signal Enable"]
pub type Sigen1p8vW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Drvstnsel {
    #[doc = "0: Driver Type B is selected (Default)"]
    Typeb = 0,
    #[doc = "1: Driver Type A is selected"]
    Typea = 1,
    #[doc = "2: Driver Type C is selected"]
    Typec = 2,
    #[doc = "3: Driver Type D is selected"]
    Typed = 3,
}
impl From<Drvstnsel> for u8 {
    #[inline(always)]
    fn from(variant: Drvstnsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Drvstnsel {
    type Ux = u8;
}
impl crate::IsEnum for Drvstnsel {}
#[doc = "Field `DRVSTNSEL` reader - Driver Strength Select"]
pub type DrvstnselR = crate::FieldReader<Drvstnsel>;
impl DrvstnselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drvstnsel {
        match self.bits {
            0 => Drvstnsel::Typeb,
            1 => Drvstnsel::Typea,
            2 => Drvstnsel::Typec,
            3 => Drvstnsel::Typed,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == Drvstnsel::Typeb
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == Drvstnsel::Typea
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == Drvstnsel::Typec
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == Drvstnsel::Typed
    }
}
#[doc = "Field `DRVSTNSEL` writer - Driver Strength Select"]
pub type DrvstnselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Drvstnsel, crate::Safe>;
impl<'a, REG> DrvstnselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstnsel::Typeb)
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstnsel::Typea)
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstnsel::Typec)
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(Drvstnsel::Typed)
    }
}
#[doc = "Field `EXETUNING` reader - Execute Tuning"]
pub type ExetuningR = crate::BitReader;
#[doc = "Field `EXETUNING` writer - Execute Tuning"]
pub type ExetuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPCLKSEL` reader - Sampling Clock Select"]
pub type SampclkselR = crate::BitReader;
#[doc = "Field `SAMPCLKSEL` writer - Sampling Clock Select"]
pub type SampclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCINTEN` reader - Asynchronous Interrupt Enable"]
pub type AsyncintenR = crate::BitReader;
#[doc = "Field `ASYNCINTEN` writer - Asynchronous Interrupt Enable"]
pub type AsyncintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSTVALEN` reader - Preset Value Enable"]
pub type PrstvalenR = crate::BitReader;
#[doc = "Field `PRSTVALEN` writer - Preset Value Enable"]
pub type PrstvalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12notexe(&self) -> Ac12notexeR {
        Ac12notexeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> Ac12toeR {
        Ac12toeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crcerr(&self) -> Ac12crcerrR {
        Ac12crcerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12endbiterr(&self) -> Ac12endbiterrR {
        Ac12endbiterrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indexerr(&self) -> Ac12indexerrR {
        Ac12indexerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12err(&self) -> Cnibac12errR {
        Cnibac12errR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsmodesel(&self) -> UhsmodeselR {
        UhsmodeselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    pub fn sigen1p8v(&self) -> Sigen1p8vR {
        Sigen1p8vR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvstnsel(&self) -> DrvstnselR {
        DrvstnselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn exetuning(&self) -> ExetuningR {
        ExetuningR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    pub fn sampclksel(&self) -> SampclkselR {
        SampclkselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asyncinten(&self) -> AsyncintenR {
        AsyncintenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    pub fn prstvalen(&self) -> PrstvalenR {
        PrstvalenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsmodesel(&mut self) -> UhsmodeselW<'_, Ac12errstatSpec> {
        UhsmodeselW::new(self, 16)
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    pub fn sigen1p8v(&mut self) -> Sigen1p8vW<'_, Ac12errstatSpec> {
        Sigen1p8vW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvstnsel(&mut self) -> DrvstnselW<'_, Ac12errstatSpec> {
        DrvstnselW::new(self, 20)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn exetuning(&mut self) -> ExetuningW<'_, Ac12errstatSpec> {
        ExetuningW::new(self, 22)
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    pub fn sampclksel(&mut self) -> SampclkselW<'_, Ac12errstatSpec> {
        SampclkselW::new(self, 23)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asyncinten(&mut self) -> AsyncintenW<'_, Ac12errstatSpec> {
        AsyncintenW::new(self, 30)
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    pub fn prstvalen(&mut self) -> PrstvalenW<'_, Ac12errstatSpec> {
        PrstvalenW::new(self, 31)
    }
}
#[doc = "AUTO CMD12 Error Status and Host Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ac12errstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ac12errstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ac12errstatSpec;
impl crate::RegisterSpec for Ac12errstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac12errstat::R`](R) reader structure"]
impl crate::Readable for Ac12errstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ac12errstat::W`](W) writer structure"]
impl crate::Writable for Ac12errstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AC12ERRSTAT to value 0"]
impl crate::Resettable for Ac12errstatSpec {}
