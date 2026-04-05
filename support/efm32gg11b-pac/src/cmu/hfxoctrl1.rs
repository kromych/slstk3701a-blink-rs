#[doc = "Register `HFXOCTRL1` reader"]
pub type R = crate::R<Hfxoctrl1Spec>;
#[doc = "Register `HFXOCTRL1` writer"]
pub type W = crate::W<Hfxoctrl1Spec>;
#[doc = "Sets the Amplitude Detection Level (mV)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Peakdetthr {
    #[doc = "0: 50mV amplitude detection level"]
    Thr0 = 0,
    #[doc = "1: 75mV amplitude detection level"]
    Thr1 = 1,
    #[doc = "2: 115mV amplitude detection level"]
    Thr2 = 2,
    #[doc = "3: 160mV amplitude detection level"]
    Thr3 = 3,
    #[doc = "4: 220mV amplitude detection level"]
    Thr4 = 4,
    #[doc = "5: 260mV amplitude detection level"]
    Thr5 = 5,
    #[doc = "6: 320mV amplitude detection level"]
    Thr6 = 6,
    #[doc = "7: Same as THR6"]
    Thr7 = 7,
}
impl From<Peakdetthr> for u8 {
    #[inline(always)]
    fn from(variant: Peakdetthr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Peakdetthr {
    type Ux = u8;
}
impl crate::IsEnum for Peakdetthr {}
#[doc = "Field `PEAKDETTHR` reader - Sets the Amplitude Detection Level (mV)"]
pub type PeakdetthrR = crate::FieldReader<Peakdetthr>;
impl PeakdetthrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peakdetthr {
        match self.bits {
            0 => Peakdetthr::Thr0,
            1 => Peakdetthr::Thr1,
            2 => Peakdetthr::Thr2,
            3 => Peakdetthr::Thr3,
            4 => Peakdetthr::Thr4,
            5 => Peakdetthr::Thr5,
            6 => Peakdetthr::Thr6,
            7 => Peakdetthr::Thr7,
            _ => unreachable!(),
        }
    }
    #[doc = "50mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr0(&self) -> bool {
        *self == Peakdetthr::Thr0
    }
    #[doc = "75mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr1(&self) -> bool {
        *self == Peakdetthr::Thr1
    }
    #[doc = "115mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr2(&self) -> bool {
        *self == Peakdetthr::Thr2
    }
    #[doc = "160mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr3(&self) -> bool {
        *self == Peakdetthr::Thr3
    }
    #[doc = "220mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr4(&self) -> bool {
        *self == Peakdetthr::Thr4
    }
    #[doc = "260mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr5(&self) -> bool {
        *self == Peakdetthr::Thr5
    }
    #[doc = "320mV amplitude detection level"]
    #[inline(always)]
    pub fn is_thr6(&self) -> bool {
        *self == Peakdetthr::Thr6
    }
    #[doc = "Same as THR6"]
    #[inline(always)]
    pub fn is_thr7(&self) -> bool {
        *self == Peakdetthr::Thr7
    }
}
#[doc = "Field `PEAKDETTHR` writer - Sets the Amplitude Detection Level (mV)"]
pub type PeakdetthrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Peakdetthr, crate::Safe>;
impl<'a, REG> PeakdetthrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "50mV amplitude detection level"]
    #[inline(always)]
    pub fn thr0(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr0)
    }
    #[doc = "75mV amplitude detection level"]
    #[inline(always)]
    pub fn thr1(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr1)
    }
    #[doc = "115mV amplitude detection level"]
    #[inline(always)]
    pub fn thr2(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr2)
    }
    #[doc = "160mV amplitude detection level"]
    #[inline(always)]
    pub fn thr3(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr3)
    }
    #[doc = "220mV amplitude detection level"]
    #[inline(always)]
    pub fn thr4(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr4)
    }
    #[doc = "260mV amplitude detection level"]
    #[inline(always)]
    pub fn thr5(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr5)
    }
    #[doc = "320mV amplitude detection level"]
    #[inline(always)]
    pub fn thr6(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr6)
    }
    #[doc = "Same as THR6"]
    #[inline(always)]
    pub fn thr7(self) -> &'a mut crate::W<REG> {
        self.variant(Peakdetthr::Thr7)
    }
}
impl R {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PeakdetthrR {
        PeakdetthrR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&mut self) -> PeakdetthrW<'_, Hfxoctrl1Spec> {
        PeakdetthrW::new(self, 12)
    }
}
#[doc = "HFXO Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxoctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxoctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfxoctrl1Spec;
impl crate::RegisterSpec for Hfxoctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxoctrl1::R`](R) reader structure"]
impl crate::Readable for Hfxoctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`hfxoctrl1::W`](W) writer structure"]
impl crate::Writable for Hfxoctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFXOCTRL1 to value 0x2000"]
impl crate::Resettable for Hfxoctrl1Spec {
    const RESET_VALUE: u32 = 0x2000;
}
