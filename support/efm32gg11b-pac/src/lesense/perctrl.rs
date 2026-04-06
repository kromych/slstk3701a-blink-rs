#[doc = "Register `PERCTRL` reader"]
pub type R = crate::R<PerctrlSpec>;
#[doc = "Register `PERCTRL` writer"]
pub type W = crate::W<PerctrlSpec>;
#[doc = "Field `DACCH0EN` reader - VDAC CH0 Enable"]
pub type Dacch0enR = crate::BitReader;
#[doc = "Field `DACCH0EN` writer - VDAC CH0 Enable"]
pub type Dacch0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH1EN` reader - VDAC CH1 Enable"]
pub type Dacch1enR = crate::BitReader;
#[doc = "Field `DACCH1EN` writer - VDAC CH1 Enable"]
pub type Dacch1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH0DATA` reader - VDAC CH0 Data Selection"]
pub type Dacch0dataR = crate::BitReader;
#[doc = "Field `DACCH0DATA` writer - VDAC CH0 Data Selection"]
pub type Dacch0dataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCH1DATA` reader - VDAC CH1 Data Selection"]
pub type Dacch1dataR = crate::BitReader;
#[doc = "Field `DACCH1DATA` writer - VDAC CH1 Data Selection"]
pub type Dacch1dataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACSTARTUP` reader - VDAC Startup Configuration"]
pub type DacstartupR = crate::BitReader;
#[doc = "Field `DACSTARTUP` writer - VDAC Startup Configuration"]
pub type DacstartupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACCONVTRIG` reader - VDAC Conversion Trigger Configuration"]
pub type DacconvtrigR = crate::BitReader;
#[doc = "Field `DACCONVTRIG` writer - VDAC Conversion Trigger Configuration"]
pub type DacconvtrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ACMP0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmp0mode {
    #[doc = "0: LESENSE does not control ACMP0"]
    Disable = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP0"]
    Mux = 1,
    #[doc = "2: LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    Muxthres = 2,
}
impl From<Acmp0mode> for u8 {
    #[inline(always)]
    fn from(variant: Acmp0mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmp0mode {
    type Ux = u8;
}
impl crate::IsEnum for Acmp0mode {}
#[doc = "Field `ACMP0MODE` reader - ACMP0 Mode"]
pub type Acmp0modeR = crate::FieldReader<Acmp0mode>;
impl Acmp0modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acmp0mode> {
        match self.bits {
            0 => Some(Acmp0mode::Disable),
            1 => Some(Acmp0mode::Mux),
            2 => Some(Acmp0mode::Muxthres),
            _ => None,
        }
    }
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Acmp0mode::Disable
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == Acmp0mode::Mux
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == Acmp0mode::Muxthres
    }
}
#[doc = "Field `ACMP0MODE` writer - ACMP0 Mode"]
pub type Acmp0modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmp0mode>;
impl<'a, REG> Acmp0modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0mode::Disable)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0mode::Mux)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp0mode::Muxthres)
    }
}
#[doc = "ACMP1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Acmp1mode {
    #[doc = "0: LESENSE does not control ACMP1"]
    Disable = 0,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP1"]
    Mux = 1,
    #[doc = "2: LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    Muxthres = 2,
}
impl From<Acmp1mode> for u8 {
    #[inline(always)]
    fn from(variant: Acmp1mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Acmp1mode {
    type Ux = u8;
}
impl crate::IsEnum for Acmp1mode {}
#[doc = "Field `ACMP1MODE` reader - ACMP1 Mode"]
pub type Acmp1modeR = crate::FieldReader<Acmp1mode>;
impl Acmp1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Acmp1mode> {
        match self.bits {
            0 => Some(Acmp1mode::Disable),
            1 => Some(Acmp1mode::Mux),
            2 => Some(Acmp1mode::Muxthres),
            _ => None,
        }
    }
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Acmp1mode::Disable
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == Acmp1mode::Mux
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == Acmp1mode::Muxthres
    }
}
#[doc = "Field `ACMP1MODE` writer - ACMP1 Mode"]
pub type Acmp1modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Acmp1mode>;
impl<'a, REG> Acmp1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp1mode::Disable)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp1mode::Mux)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut crate::W<REG> {
        self.variant(Acmp1mode::Muxthres)
    }
}
#[doc = "Field `ACMP0INV` reader - Invert Analog Comparator 0 Output"]
pub type Acmp0invR = crate::BitReader;
#[doc = "Field `ACMP0INV` writer - Invert Analog Comparator 0 Output"]
pub type Acmp0invW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1INV` reader - Invert Analog Comparator 1 Output"]
pub type Acmp1invR = crate::BitReader;
#[doc = "Field `ACMP1INV` writer - Invert Analog Comparator 1 Output"]
pub type Acmp1invW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0HYSTEN` reader - ACMP0 Hysteresis Enable"]
pub type Acmp0hystenR = crate::BitReader;
#[doc = "Field `ACMP0HYSTEN` writer - ACMP0 Hysteresis Enable"]
pub type Acmp0hystenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1HYSTEN` reader - ACMP1 Hysteresis Enable"]
pub type Acmp1hystenR = crate::BitReader;
#[doc = "Field `ACMP1HYSTEN` writer - ACMP1 Hysteresis Enable"]
pub type Acmp1hystenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ACMP and VDAC Duty Cycle Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmupmode {
    #[doc = "0: The analog comparators and VDAC are shut down when LESENSE is idle"]
    Normal = 0,
    #[doc = "1: The analog comparators are kept powered up when LESENSE is idle"]
    Keepacmpwarm = 1,
    #[doc = "2: The VDAC is kept powered up when LESENSE is idle"]
    Keepdacwarm = 2,
    #[doc = "3: The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    Keepacmpdacwarm = 3,
}
impl From<Warmupmode> for u8 {
    #[inline(always)]
    fn from(variant: Warmupmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warmupmode {
    type Ux = u8;
}
impl crate::IsEnum for Warmupmode {}
#[doc = "Field `WARMUPMODE` reader - ACMP and VDAC Duty Cycle Mode"]
pub type WarmupmodeR = crate::FieldReader<Warmupmode>;
impl WarmupmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Warmupmode {
        match self.bits {
            0 => Warmupmode::Normal,
            1 => Warmupmode::Keepacmpwarm,
            2 => Warmupmode::Keepdacwarm,
            3 => Warmupmode::Keepacmpdacwarm,
            _ => unreachable!(),
        }
    }
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Warmupmode::Normal
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == Warmupmode::Keepacmpwarm
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == Warmupmode::Keepdacwarm
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == Warmupmode::Keepacmpdacwarm
    }
}
#[doc = "Field `WARMUPMODE` writer - ACMP and VDAC Duty Cycle Mode"]
pub type WarmupmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Warmupmode, crate::Safe>;
impl<'a, REG> WarmupmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Normal)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepacmpwarm)
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepdacwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepdacwarm)
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpdacwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepacmpdacwarm)
    }
}
impl R {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    pub fn dacch0en(&self) -> Dacch0enR {
        Dacch0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    pub fn dacch1en(&self) -> Dacch1enR {
        Dacch1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    pub fn dacch0data(&self) -> Dacch0dataR {
        Dacch0dataR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    pub fn dacch1data(&self) -> Dacch1dataR {
        Dacch1dataR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    pub fn dacstartup(&self) -> DacstartupR {
        DacstartupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&self) -> DacconvtrigR {
        DacconvtrigR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> Acmp0modeR {
        Acmp0modeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> Acmp1modeR {
        Acmp1modeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> Acmp0invR {
        Acmp0invR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> Acmp1invR {
        Acmp1invR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp0hysten(&self) -> Acmp0hystenR {
        Acmp0hystenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp1hysten(&self) -> Acmp1hystenR {
        Acmp1hystenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    pub fn dacch0en(&mut self) -> Dacch0enW<'_, PerctrlSpec> {
        Dacch0enW::new(self, 0)
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    pub fn dacch1en(&mut self) -> Dacch1enW<'_, PerctrlSpec> {
        Dacch1enW::new(self, 1)
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    pub fn dacch0data(&mut self) -> Dacch0dataW<'_, PerctrlSpec> {
        Dacch0dataW::new(self, 2)
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    pub fn dacch1data(&mut self) -> Dacch1dataW<'_, PerctrlSpec> {
        Dacch1dataW::new(self, 3)
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    pub fn dacstartup(&mut self) -> DacstartupW<'_, PerctrlSpec> {
        DacstartupW::new(self, 6)
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&mut self) -> DacconvtrigW<'_, PerctrlSpec> {
        DacconvtrigW::new(self, 8)
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    pub fn acmp0mode(&mut self) -> Acmp0modeW<'_, PerctrlSpec> {
        Acmp0modeW::new(self, 20)
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    pub fn acmp1mode(&mut self) -> Acmp1modeW<'_, PerctrlSpec> {
        Acmp1modeW::new(self, 22)
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn acmp0inv(&mut self) -> Acmp0invW<'_, PerctrlSpec> {
        Acmp0invW::new(self, 24)
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn acmp1inv(&mut self) -> Acmp1invW<'_, PerctrlSpec> {
        Acmp1invW::new(self, 25)
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp0hysten(&mut self) -> Acmp0hystenW<'_, PerctrlSpec> {
        Acmp0hystenW::new(self, 26)
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp1hysten(&mut self) -> Acmp1hystenW<'_, PerctrlSpec> {
        Acmp1hystenW::new(self, 27)
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WarmupmodeW<'_, PerctrlSpec> {
        WarmupmodeW::new(self, 28)
    }
}
#[doc = "Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`perctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerctrlSpec;
impl crate::RegisterSpec for PerctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perctrl::R`](R) reader structure"]
impl crate::Readable for PerctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`perctrl::W`](W) writer structure"]
impl crate::Writable for PerctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERCTRL to value 0"]
impl crate::Resettable for PerctrlSpec {}
