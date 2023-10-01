#[doc = "Register `AC12ERRSTAT` reader"]
pub type R = crate::R<AC12ERRSTAT_SPEC>;
#[doc = "Register `AC12ERRSTAT` writer"]
pub type W = crate::W<AC12ERRSTAT_SPEC>;
#[doc = "Field `AC12NOTEXE` reader - Auto CMD12 Not Executed"]
pub type AC12NOTEXE_R = crate::BitReader;
#[doc = "Field `AC12TOE` reader - Auto CMD12 Timeout Error"]
pub type AC12TOE_R = crate::BitReader;
#[doc = "Field `AC12CRCERR` reader - Auto CMD CRC Error"]
pub type AC12CRCERR_R = crate::BitReader;
#[doc = "Field `AC12ENDBITERR` reader - Auto CMD End Bit Error"]
pub type AC12ENDBITERR_R = crate::BitReader;
#[doc = "Field `AC12INDEXERR` reader - Auto CMD Index Error"]
pub type AC12INDEXERR_R = crate::BitReader;
#[doc = "Field `CNIBAC12ERR` reader - Command Not Issued By Auto CMD12 Error"]
pub type CNIBAC12ERR_R = crate::BitReader;
#[doc = "Field `UHSMODESEL` reader - UHS Mode Select"]
pub type UHSMODESEL_R = crate::FieldReader<UHSMODESEL_A>;
#[doc = "UHS Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UHSMODESEL_A {
    #[doc = "0: SDR12"]
    SDR12 = 0,
    #[doc = "1: SDR25"]
    SDR25 = 1,
    #[doc = "2: SDR50"]
    SDR50 = 2,
    #[doc = "3: SDR104"]
    SDR104 = 3,
    #[doc = "4: DDR50"]
    DDR50 = 4,
}
impl From<UHSMODESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UHSMODESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UHSMODESEL_A {
    type Ux = u8;
}
impl UHSMODESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UHSMODESEL_A> {
        match self.bits {
            0 => Some(UHSMODESEL_A::SDR12),
            1 => Some(UHSMODESEL_A::SDR25),
            2 => Some(UHSMODESEL_A::SDR50),
            3 => Some(UHSMODESEL_A::SDR104),
            4 => Some(UHSMODESEL_A::DDR50),
            _ => None,
        }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMODESEL_A::SDR12
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMODESEL_A::SDR25
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMODESEL_A::SDR50
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMODESEL_A::SDR104
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMODESEL_A::DDR50
    }
}
#[doc = "Field `UHSMODESEL` writer - UHS Mode Select"]
pub type UHSMODESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, UHSMODESEL_A>;
impl<'a, REG, const O: u8> UHSMODESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODESEL_A::SDR12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODESEL_A::SDR25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODESEL_A::SDR50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODESEL_A::SDR104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut crate::W<REG> {
        self.variant(UHSMODESEL_A::DDR50)
    }
}
#[doc = "Field `SIGEN1P8V` reader - Voltage 1.8V Signal Enable"]
pub type SIGEN1P8V_R = crate::BitReader;
#[doc = "Field `SIGEN1P8V` writer - Voltage 1.8V Signal Enable"]
pub type SIGEN1P8V_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRVSTNSEL` reader - Driver Strength Select"]
pub type DRVSTNSEL_R = crate::FieldReader<DRVSTNSEL_A>;
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRVSTNSEL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<DRVSTNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSTNSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DRVSTNSEL_A {
    type Ux = u8;
}
impl DRVSTNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSTNSEL_A {
        match self.bits {
            0 => DRVSTNSEL_A::TYPEB,
            1 => DRVSTNSEL_A::TYPEA,
            2 => DRVSTNSEL_A::TYPEC,
            3 => DRVSTNSEL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEB
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEA
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEC
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == DRVSTNSEL_A::TYPED
    }
}
#[doc = "Field `DRVSTNSEL` writer - Driver Strength Select"]
pub type DRVSTNSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, DRVSTNSEL_A>;
impl<'a, REG, const O: u8> DRVSTNSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSTNSEL_A::TYPEB)
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSTNSEL_A::TYPEA)
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSTNSEL_A::TYPEC)
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut crate::W<REG> {
        self.variant(DRVSTNSEL_A::TYPED)
    }
}
#[doc = "Field `EXETUNING` reader - Execute Tuning"]
pub type EXETUNING_R = crate::BitReader;
#[doc = "Field `EXETUNING` writer - Execute Tuning"]
pub type EXETUNING_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAMPCLKSEL` reader - Sampling Clock Select"]
pub type SAMPCLKSEL_R = crate::BitReader;
#[doc = "Field `SAMPCLKSEL` writer - Sampling Clock Select"]
pub type SAMPCLKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCINTEN` reader - Asynchronous Interrupt Enable"]
pub type ASYNCINTEN_R = crate::BitReader;
#[doc = "Field `ASYNCINTEN` writer - Asynchronous Interrupt Enable"]
pub type ASYNCINTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRSTVALEN` reader - Preset Value Enable"]
pub type PRSTVALEN_R = crate::BitReader;
#[doc = "Field `PRSTVALEN` writer - Preset Value Enable"]
pub type PRSTVALEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12notexe(&self) -> AC12NOTEXE_R {
        AC12NOTEXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crcerr(&self) -> AC12CRCERR_R {
        AC12CRCERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12endbiterr(&self) -> AC12ENDBITERR_R {
        AC12ENDBITERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indexerr(&self) -> AC12INDEXERR_R {
        AC12INDEXERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12err(&self) -> CNIBAC12ERR_R {
        CNIBAC12ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsmodesel(&self) -> UHSMODESEL_R {
        UHSMODESEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    pub fn sigen1p8v(&self) -> SIGEN1P8V_R {
        SIGEN1P8V_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvstnsel(&self) -> DRVSTNSEL_R {
        DRVSTNSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn exetuning(&self) -> EXETUNING_R {
        EXETUNING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    pub fn sampclksel(&self) -> SAMPCLKSEL_R {
        SAMPCLKSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asyncinten(&self) -> ASYNCINTEN_R {
        ASYNCINTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    pub fn prstvalen(&self) -> PRSTVALEN_R {
        PRSTVALEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn uhsmodesel(&mut self) -> UHSMODESEL_W<AC12ERRSTAT_SPEC, 16> {
        UHSMODESEL_W::new(self)
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sigen1p8v(&mut self) -> SIGEN1P8V_W<AC12ERRSTAT_SPEC, 19> {
        SIGEN1P8V_W::new(self)
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    #[must_use]
    pub fn drvstnsel(&mut self) -> DRVSTNSEL_W<AC12ERRSTAT_SPEC, 20> {
        DRVSTNSEL_W::new(self)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn exetuning(&mut self) -> EXETUNING_W<AC12ERRSTAT_SPEC, 22> {
        EXETUNING_W::new(self)
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn sampclksel(&mut self) -> SAMPCLKSEL_W<AC12ERRSTAT_SPEC, 23> {
        SAMPCLKSEL_W::new(self)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn asyncinten(&mut self) -> ASYNCINTEN_W<AC12ERRSTAT_SPEC, 30> {
        ASYNCINTEN_W::new(self)
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prstvalen(&mut self) -> PRSTVALEN_W<AC12ERRSTAT_SPEC, 31> {
        PRSTVALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AUTO CMD12 Error Status and Host Control2 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac12errstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac12errstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AC12ERRSTAT_SPEC;
impl crate::RegisterSpec for AC12ERRSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ac12errstat::R`](R) reader structure"]
impl crate::Readable for AC12ERRSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ac12errstat::W`](W) writer structure"]
impl crate::Writable for AC12ERRSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AC12ERRSTAT to value 0"]
impl crate::Resettable for AC12ERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
