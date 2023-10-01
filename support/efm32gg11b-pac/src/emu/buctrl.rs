#[doc = "Register `BUCTRL` reader"]
pub type R = crate::R<BUCTRL_SPEC>;
#[doc = "Register `BUCTRL` writer"]
pub type W = crate::W<BUCTRL_SPEC>;
#[doc = "Field `EN` reader - Enable Backup Mode"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable Backup Mode"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STATEN` reader - Enable Backup Mode Status Export"]
pub type STATEN_R = crate::BitReader;
#[doc = "Field `STATEN` writer - Enable Backup Mode Status Export"]
pub type STATEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUVINPROBEEN` reader - Enable BU_VIN Probing"]
pub type BUVINPROBEEN_R = crate::BitReader;
#[doc = "Field `BUVINPROBEEN` writer - Enable BU_VIN Probing"]
pub type BUVINPROBEEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VOUTRES` reader - BU_VOUT Resistor Select"]
pub type VOUTRES_R = crate::FieldReader<VOUTRES_A>;
#[doc = "BU_VOUT Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOUTRES_A {
    #[doc = "0: BU_VOUT is not connected"]
    DIS = 0,
    #[doc = "1: Enable weak switch between BU_VOUT and backup domain power supply."]
    WEAK = 1,
    #[doc = "2: Enable medium switch between BU_VOUT and backup domain power supply."]
    MED = 2,
    #[doc = "3: Enable strong switch between BU_VOUT and backup domain power supply."]
    STRONG = 3,
}
impl From<VOUTRES_A> for u8 {
    #[inline(always)]
    fn from(variant: VOUTRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VOUTRES_A {
    type Ux = u8;
}
impl VOUTRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUTRES_A {
        match self.bits {
            0 => VOUTRES_A::DIS,
            1 => VOUTRES_A::WEAK,
            2 => VOUTRES_A::MED,
            3 => VOUTRES_A::STRONG,
            _ => unreachable!(),
        }
    }
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VOUTRES_A::DIS
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_weak(&self) -> bool {
        *self == VOUTRES_A::WEAK
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == VOUTRES_A::MED
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == VOUTRES_A::STRONG
    }
}
#[doc = "Field `VOUTRES` writer - BU_VOUT Resistor Select"]
pub type VOUTRES_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, VOUTRES_A>;
impl<'a, REG, const O: u8> VOUTRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(VOUTRES_A::DIS)
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn weak(self) -> &'a mut crate::W<REG> {
        self.variant(VOUTRES_A::WEAK)
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(VOUTRES_A::MED)
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn strong(self) -> &'a mut crate::W<REG> {
        self.variant(VOUTRES_A::STRONG)
    }
}
#[doc = "Field `PWRRES` reader - Power Domain Resistor Select"]
pub type PWRRES_R = crate::FieldReader<PWRRES_A>;
#[doc = "Power Domain Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRRES_A {
    #[doc = "0: Main power and backup power connected with RES0 series resistance."]
    RES0 = 0,
    #[doc = "1: Main power and backup power connected with RES1 series resistance."]
    RES1 = 1,
    #[doc = "2: Main power and backup power connected with RES2 series resistance."]
    RES2 = 2,
    #[doc = "3: Main power and backup power connected with RES3 series resistance."]
    RES3 = 3,
}
impl From<PWRRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRRES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRRES_A {
    type Ux = u8;
}
impl PWRRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRRES_A {
        match self.bits {
            0 => PWRRES_A::RES0,
            1 => PWRRES_A::RES1,
            2 => PWRRES_A::RES2,
            3 => PWRRES_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == PWRRES_A::RES0
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == PWRRES_A::RES1
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == PWRRES_A::RES2
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == PWRRES_A::RES3
    }
}
#[doc = "Field `PWRRES` writer - Power Domain Resistor Select"]
pub type PWRRES_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PWRRES_A>;
impl<'a, REG, const O: u8> PWRRES_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRES_A::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRES_A::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRES_A::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(PWRRES_A::RES3)
    }
}
#[doc = "Field `BUACTPWRCON` reader - Power Connection Configuration in Backup Mode"]
pub type BUACTPWRCON_R = crate::FieldReader<BUACTPWRCON_A>;
#[doc = "Power Connection Configuration in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUACTPWRCON_A {
    #[doc = "0: No connection."]
    NONE = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    NODIODE = 3,
}
impl From<BUACTPWRCON_A> for u8 {
    #[inline(always)]
    fn from(variant: BUACTPWRCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUACTPWRCON_A {
    type Ux = u8;
}
impl BUACTPWRCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUACTPWRCON_A {
        match self.bits {
            0 => BUACTPWRCON_A::NONE,
            1 => BUACTPWRCON_A::MAINBU,
            2 => BUACTPWRCON_A::BUMAIN,
            3 => BUACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUACTPWRCON_A::NONE
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUACTPWRCON_A::MAINBU
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUACTPWRCON_A::BUMAIN
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUACTPWRCON_A::NODIODE
    }
}
#[doc = "Field `BUACTPWRCON` writer - Power Connection Configuration in Backup Mode"]
pub type BUACTPWRCON_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, BUACTPWRCON_A>;
impl<'a, REG, const O: u8> BUACTPWRCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(BUACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut crate::W<REG> {
        self.variant(BUACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut crate::W<REG> {
        self.variant(BUACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut crate::W<REG> {
        self.variant(BUACTPWRCON_A::NODIODE)
    }
}
#[doc = "Field `BUINACTPWRCON` reader - Power Connection Configuration When Not in Backup Mode"]
pub type BUINACTPWRCON_R = crate::FieldReader<BUINACTPWRCON_A>;
#[doc = "Power Connection Configuration When Not in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUINACTPWRCON_A {
    #[doc = "0: No connection."]
    NONE = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    MAINBU = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    BUMAIN = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    NODIODE = 3,
}
impl From<BUINACTPWRCON_A> for u8 {
    #[inline(always)]
    fn from(variant: BUINACTPWRCON_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUINACTPWRCON_A {
    type Ux = u8;
}
impl BUINACTPWRCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUINACTPWRCON_A {
        match self.bits {
            0 => BUINACTPWRCON_A::NONE,
            1 => BUINACTPWRCON_A::MAINBU,
            2 => BUINACTPWRCON_A::BUMAIN,
            3 => BUINACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUINACTPWRCON_A::NONE
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUINACTPWRCON_A::MAINBU
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUINACTPWRCON_A::BUMAIN
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUINACTPWRCON_A::NODIODE
    }
}
#[doc = "Field `BUINACTPWRCON` writer - Power Connection Configuration When Not in Backup Mode"]
pub type BUINACTPWRCON_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, BUINACTPWRCON_A>;
impl<'a, REG, const O: u8> BUINACTPWRCON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(BUINACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut crate::W<REG> {
        self.variant(BUINACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut crate::W<REG> {
        self.variant(BUINACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut crate::W<REG> {
        self.variant(BUINACTPWRCON_A::NODIODE)
    }
}
#[doc = "Field `DISMAXCOMP` reader - Disable MAIN-BU Comparator"]
pub type DISMAXCOMP_R = crate::BitReader;
#[doc = "Field `DISMAXCOMP` writer - Disable MAIN-BU Comparator"]
pub type DISMAXCOMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&self) -> STATEN_R {
        STATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&self) -> BUVINPROBEEN_R {
        BUVINPROBEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&self) -> VOUTRES_R {
        VOUTRES_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&self) -> PWRRES_R {
        PWRRES_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&self) -> BUACTPWRCON_R {
        BUACTPWRCON_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&self) -> BUINACTPWRCON_R {
        BUINACTPWRCON_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&self) -> DISMAXCOMP_R {
        DISMAXCOMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BUCTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    #[must_use]
    pub fn staten(&mut self) -> STATEN_W<BUCTRL_SPEC, 1> {
        STATEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    #[must_use]
    pub fn buvinprobeen(&mut self) -> BUVINPROBEEN_W<BUCTRL_SPEC, 2> {
        BUVINPROBEEN_W::new(self)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    #[must_use]
    pub fn voutres(&mut self) -> VOUTRES_W<BUCTRL_SPEC, 8> {
        VOUTRES_W::new(self)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrres(&mut self) -> PWRRES_W<BUCTRL_SPEC, 12> {
        PWRRES_W::new(self)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn buactpwrcon(&mut self) -> BUACTPWRCON_W<BUCTRL_SPEC, 16> {
        BUACTPWRCON_W::new(self)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    #[must_use]
    pub fn buinactpwrcon(&mut self) -> BUINACTPWRCON_W<BUCTRL_SPEC, 20> {
        BUINACTPWRCON_W::new(self)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    #[must_use]
    pub fn dismaxcomp(&mut self) -> DISMAXCOMP_W<BUCTRL_SPEC, 31> {
        DISMAXCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Backup Power Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUCTRL_SPEC;
impl crate::RegisterSpec for BUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buctrl::R`](R) reader structure"]
impl crate::Readable for BUCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buctrl::W`](W) writer structure"]
impl crate::Writable for BUCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUCTRL to value 0"]
impl crate::Resettable for BUCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
