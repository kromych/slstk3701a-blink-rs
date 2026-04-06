#[doc = "Register `BUCTRL` reader"]
pub type R = crate::R<BuctrlSpec>;
#[doc = "Register `BUCTRL` writer"]
pub type W = crate::W<BuctrlSpec>;
#[doc = "Field `EN` reader - Enable Backup Mode"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable Backup Mode"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATEN` reader - Enable Backup Mode Status Export"]
pub type StatenR = crate::BitReader;
#[doc = "Field `STATEN` writer - Enable Backup Mode Status Export"]
pub type StatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUVINPROBEEN` reader - Enable BU_VIN Probing"]
pub type BuvinprobeenR = crate::BitReader;
#[doc = "Field `BUVINPROBEEN` writer - Enable BU_VIN Probing"]
pub type BuvinprobeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "BU_VOUT Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Voutres {
    #[doc = "0: BU_VOUT is not connected"]
    Dis = 0,
    #[doc = "1: Enable weak switch between BU_VOUT and backup domain power supply."]
    Weak = 1,
    #[doc = "2: Enable medium switch between BU_VOUT and backup domain power supply."]
    Med = 2,
    #[doc = "3: Enable strong switch between BU_VOUT and backup domain power supply."]
    Strong = 3,
}
impl From<Voutres> for u8 {
    #[inline(always)]
    fn from(variant: Voutres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Voutres {
    type Ux = u8;
}
impl crate::IsEnum for Voutres {}
#[doc = "Field `VOUTRES` reader - BU_VOUT Resistor Select"]
pub type VoutresR = crate::FieldReader<Voutres>;
impl VoutresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voutres {
        match self.bits {
            0 => Voutres::Dis,
            1 => Voutres::Weak,
            2 => Voutres::Med,
            3 => Voutres::Strong,
            _ => unreachable!(),
        }
    }
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Voutres::Dis
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_weak(&self) -> bool {
        *self == Voutres::Weak
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == Voutres::Med
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == Voutres::Strong
    }
}
#[doc = "Field `VOUTRES` writer - BU_VOUT Resistor Select"]
pub type VoutresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Voutres, crate::Safe>;
impl<'a, REG> VoutresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Voutres::Dis)
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn weak(self) -> &'a mut crate::W<REG> {
        self.variant(Voutres::Weak)
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(Voutres::Med)
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn strong(self) -> &'a mut crate::W<REG> {
        self.variant(Voutres::Strong)
    }
}
#[doc = "Power Domain Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pwrres {
    #[doc = "0: Main power and backup power connected with RES0 series resistance."]
    Res0 = 0,
    #[doc = "1: Main power and backup power connected with RES1 series resistance."]
    Res1 = 1,
    #[doc = "2: Main power and backup power connected with RES2 series resistance."]
    Res2 = 2,
    #[doc = "3: Main power and backup power connected with RES3 series resistance."]
    Res3 = 3,
}
impl From<Pwrres> for u8 {
    #[inline(always)]
    fn from(variant: Pwrres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pwrres {
    type Ux = u8;
}
impl crate::IsEnum for Pwrres {}
#[doc = "Field `PWRRES` reader - Power Domain Resistor Select"]
pub type PwrresR = crate::FieldReader<Pwrres>;
impl PwrresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrres {
        match self.bits {
            0 => Pwrres::Res0,
            1 => Pwrres::Res1,
            2 => Pwrres::Res2,
            3 => Pwrres::Res3,
            _ => unreachable!(),
        }
    }
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == Pwrres::Res0
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == Pwrres::Res1
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == Pwrres::Res2
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == Pwrres::Res3
    }
}
#[doc = "Field `PWRRES` writer - Power Domain Resistor Select"]
pub type PwrresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pwrres, crate::Safe>;
impl<'a, REG> PwrresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrres::Res0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrres::Res1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrres::Res2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrres::Res3)
    }
}
#[doc = "Power Connection Configuration in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Buactpwrcon {
    #[doc = "0: No connection."]
    None = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    Mainbu = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    Bumain = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    Nodiode = 3,
}
impl From<Buactpwrcon> for u8 {
    #[inline(always)]
    fn from(variant: Buactpwrcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Buactpwrcon {
    type Ux = u8;
}
impl crate::IsEnum for Buactpwrcon {}
#[doc = "Field `BUACTPWRCON` reader - Power Connection Configuration in Backup Mode"]
pub type BuactpwrconR = crate::FieldReader<Buactpwrcon>;
impl BuactpwrconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buactpwrcon {
        match self.bits {
            0 => Buactpwrcon::None,
            1 => Buactpwrcon::Mainbu,
            2 => Buactpwrcon::Bumain,
            3 => Buactpwrcon::Nodiode,
            _ => unreachable!(),
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Buactpwrcon::None
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == Buactpwrcon::Mainbu
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == Buactpwrcon::Bumain
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == Buactpwrcon::Nodiode
    }
}
#[doc = "Field `BUACTPWRCON` writer - Power Connection Configuration in Backup Mode"]
pub type BuactpwrconW<'a, REG> = crate::FieldWriter<'a, REG, 2, Buactpwrcon, crate::Safe>;
impl<'a, REG> BuactpwrconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Buactpwrcon::None)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut crate::W<REG> {
        self.variant(Buactpwrcon::Mainbu)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut crate::W<REG> {
        self.variant(Buactpwrcon::Bumain)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut crate::W<REG> {
        self.variant(Buactpwrcon::Nodiode)
    }
}
#[doc = "Power Connection Configuration When Not in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Buinactpwrcon {
    #[doc = "0: No connection."]
    None = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    Mainbu = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    Bumain = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    Nodiode = 3,
}
impl From<Buinactpwrcon> for u8 {
    #[inline(always)]
    fn from(variant: Buinactpwrcon) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Buinactpwrcon {
    type Ux = u8;
}
impl crate::IsEnum for Buinactpwrcon {}
#[doc = "Field `BUINACTPWRCON` reader - Power Connection Configuration When Not in Backup Mode"]
pub type BuinactpwrconR = crate::FieldReader<Buinactpwrcon>;
impl BuinactpwrconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Buinactpwrcon {
        match self.bits {
            0 => Buinactpwrcon::None,
            1 => Buinactpwrcon::Mainbu,
            2 => Buinactpwrcon::Bumain,
            3 => Buinactpwrcon::Nodiode,
            _ => unreachable!(),
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Buinactpwrcon::None
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == Buinactpwrcon::Mainbu
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == Buinactpwrcon::Bumain
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == Buinactpwrcon::Nodiode
    }
}
#[doc = "Field `BUINACTPWRCON` writer - Power Connection Configuration When Not in Backup Mode"]
pub type BuinactpwrconW<'a, REG> = crate::FieldWriter<'a, REG, 2, Buinactpwrcon, crate::Safe>;
impl<'a, REG> BuinactpwrconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Buinactpwrcon::None)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut crate::W<REG> {
        self.variant(Buinactpwrcon::Mainbu)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut crate::W<REG> {
        self.variant(Buinactpwrcon::Bumain)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut crate::W<REG> {
        self.variant(Buinactpwrcon::Nodiode)
    }
}
#[doc = "Field `DISMAXCOMP` reader - Disable MAIN-BU Comparator"]
pub type DismaxcompR = crate::BitReader;
#[doc = "Field `DISMAXCOMP` writer - Disable MAIN-BU Comparator"]
pub type DismaxcompW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&self) -> StatenR {
        StatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&self) -> BuvinprobeenR {
        BuvinprobeenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&self) -> VoutresR {
        VoutresR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&self) -> PwrresR {
        PwrresR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&self) -> BuactpwrconR {
        BuactpwrconR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&self) -> BuinactpwrconR {
        BuinactpwrconR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&self) -> DismaxcompR {
        DismaxcompR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, BuctrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&mut self) -> StatenW<'_, BuctrlSpec> {
        StatenW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&mut self) -> BuvinprobeenW<'_, BuctrlSpec> {
        BuvinprobeenW::new(self, 2)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&mut self) -> VoutresW<'_, BuctrlSpec> {
        VoutresW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&mut self) -> PwrresW<'_, BuctrlSpec> {
        PwrresW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&mut self) -> BuactpwrconW<'_, BuctrlSpec> {
        BuactpwrconW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&mut self) -> BuinactpwrconW<'_, BuctrlSpec> {
        BuinactpwrconW::new(self, 20)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&mut self) -> DismaxcompW<'_, BuctrlSpec> {
        DismaxcompW::new(self, 31)
    }
}
#[doc = "Backup Power Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuctrlSpec;
impl crate::RegisterSpec for BuctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buctrl::R`](R) reader structure"]
impl crate::Readable for BuctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`buctrl::W`](W) writer structure"]
impl crate::Writable for BuctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUCTRL to value 0"]
impl crate::Resettable for BuctrlSpec {}
