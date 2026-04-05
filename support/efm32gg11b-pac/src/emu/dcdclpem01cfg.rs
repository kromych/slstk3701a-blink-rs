#[doc = "Register `DCDCLPEM01CFG` reader"]
pub type R = crate::R<Dcdclpem01cfgSpec>;
#[doc = "Register `DCDCLPEM01CFG` writer"]
pub type W = crate::W<Dcdclpem01cfgSpec>;
#[doc = "LP Mode Comparator Bias Selection for EM01\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpcmpbiasem01 {
    #[doc = "0: Maximum load current less than 75uA."]
    Bias0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    Bias1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    Bias2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    Bias3 = 3,
}
impl From<Lpcmpbiasem01> for u8 {
    #[inline(always)]
    fn from(variant: Lpcmpbiasem01) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpcmpbiasem01 {
    type Ux = u8;
}
impl crate::IsEnum for Lpcmpbiasem01 {}
#[doc = "Field `LPCMPBIASEM01` reader - LP Mode Comparator Bias Selection for EM01"]
pub type Lpcmpbiasem01R = crate::FieldReader<Lpcmpbiasem01>;
impl Lpcmpbiasem01R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcmpbiasem01 {
        match self.bits {
            0 => Lpcmpbiasem01::Bias0,
            1 => Lpcmpbiasem01::Bias1,
            2 => Lpcmpbiasem01::Bias2,
            3 => Lpcmpbiasem01::Bias3,
            _ => unreachable!(),
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == Lpcmpbiasem01::Bias0
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == Lpcmpbiasem01::Bias1
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == Lpcmpbiasem01::Bias2
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == Lpcmpbiasem01::Bias3
    }
}
#[doc = "Field `LPCMPBIASEM01` writer - LP Mode Comparator Bias Selection for EM01"]
pub type Lpcmpbiasem01W<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpcmpbiasem01, crate::Safe>;
impl<'a, REG> Lpcmpbiasem01W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem01::Bias0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem01::Bias1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem01::Bias2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpcmpbiasem01::Bias3)
    }
}
#[doc = "Field `LPCMPHYSSELEM01` reader - LP Mode Hysteresis Selection for EM01"]
pub type Lpcmphysselem01R = crate::FieldReader;
#[doc = "Field `LPCMPHYSSELEM01` writer - LP Mode Hysteresis Selection for EM01"]
pub type Lpcmphysselem01W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&self) -> Lpcmpbiasem01R {
        Lpcmpbiasem01R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&self) -> Lpcmphysselem01R {
        Lpcmphysselem01R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&mut self) -> Lpcmpbiasem01W<'_, Dcdclpem01cfgSpec> {
        Lpcmpbiasem01W::new(self, 8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&mut self) -> Lpcmphysselem01W<'_, Dcdclpem01cfgSpec> {
        Lpcmphysselem01W::new(self, 12)
    }
}
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclpem01cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpem01cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcdclpem01cfgSpec;
impl crate::RegisterSpec for Dcdclpem01cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclpem01cfg::R`](R) reader structure"]
impl crate::Readable for Dcdclpem01cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdclpem01cfg::W`](W) writer structure"]
impl crate::Writable for Dcdclpem01cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCLPEM01CFG to value 0x0300"]
impl crate::Resettable for Dcdclpem01cfgSpec {
    const RESET_VALUE: u32 = 0x0300;
}
