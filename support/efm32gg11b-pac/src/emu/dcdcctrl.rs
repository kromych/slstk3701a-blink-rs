#[doc = "Register `DCDCCTRL` reader"]
pub type R = crate::R<DcdcctrlSpec>;
#[doc = "Register `DCDCCTRL` writer"]
pub type W = crate::W<DcdcctrlSpec>;
#[doc = "Regulator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dcdcmode {
    #[doc = "0: DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    Bypass = 0,
    #[doc = "1: DCDC regulator is operating in low noise mode."]
    Lownoise = 1,
    #[doc = "2: DCDC regulator is operating in low power mode."]
    Lowpower = 2,
    #[doc = "3: DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    Off = 3,
}
impl From<Dcdcmode> for u8 {
    #[inline(always)]
    fn from(variant: Dcdcmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dcdcmode {
    type Ux = u8;
}
impl crate::IsEnum for Dcdcmode {}
#[doc = "Field `DCDCMODE` reader - Regulator Mode"]
pub type DcdcmodeR = crate::FieldReader<Dcdcmode>;
impl DcdcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdcmode {
        match self.bits {
            0 => Dcdcmode::Bypass,
            1 => Dcdcmode::Lownoise,
            2 => Dcdcmode::Lowpower,
            3 => Dcdcmode::Off,
            _ => unreachable!(),
        }
    }
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Dcdcmode::Bypass
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn is_lownoise(&self) -> bool {
        *self == Dcdcmode::Lownoise
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == Dcdcmode::Lowpower
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Dcdcmode::Off
    }
}
#[doc = "Field `DCDCMODE` writer - Regulator Mode"]
pub type DcdcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dcdcmode, crate::Safe>;
impl<'a, REG> DcdcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcmode::Bypass)
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn lownoise(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcmode::Lownoise)
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcmode::Lowpower)
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdcmode::Off)
    }
}
#[doc = "Field `DCDCMODEEM23` reader - DCDC Mode EM23"]
pub type Dcdcmodeem23R = crate::BitReader;
#[doc = "Field `DCDCMODEEM23` writer - DCDC Mode EM23"]
pub type Dcdcmodeem23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDCMODEEM4` reader - DCDC Mode EM4H"]
pub type Dcdcmodeem4R = crate::BitReader;
#[doc = "Field `DCDCMODEEM4` writer - DCDC Mode EM4H"]
pub type Dcdcmodeem4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&self) -> DcdcmodeR {
        DcdcmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&self) -> Dcdcmodeem23R {
        Dcdcmodeem23R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&self) -> Dcdcmodeem4R {
        Dcdcmodeem4R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&mut self) -> DcdcmodeW<'_, DcdcctrlSpec> {
        DcdcmodeW::new(self, 0)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&mut self) -> Dcdcmodeem23W<'_, DcdcctrlSpec> {
        Dcdcmodeem23W::new(self, 4)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&mut self) -> Dcdcmodeem4W<'_, DcdcctrlSpec> {
        Dcdcmodeem4W::new(self, 5)
    }
}
#[doc = "DCDC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdcctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdcctrlSpec;
impl crate::RegisterSpec for DcdcctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcctrl::R`](R) reader structure"]
impl crate::Readable for DcdcctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdcctrl::W`](W) writer structure"]
impl crate::Writable for DcdcctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCCTRL to value 0x33"]
impl crate::Resettable for DcdcctrlSpec {
    const RESET_VALUE: u32 = 0x33;
}
