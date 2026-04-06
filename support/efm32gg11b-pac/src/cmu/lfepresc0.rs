#[doc = "Register `LFEPRESC0` reader"]
pub type R = crate::R<Lfepresc0Spec>;
#[doc = "Register `LFEPRESC0` writer"]
pub type W = crate::W<Lfepresc0Spec>;
#[doc = "Real-Time Counter and Calendar Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcc {
    #[doc = "0: LFECLKRTCC = LFECLK"]
    Div1 = 0,
    #[doc = "1: LFECLKRTCC = LFECLK/2"]
    Div2 = 1,
    #[doc = "2: LFECLKRTCC = LFECLK/4"]
    Div4 = 2,
}
impl From<Rtcc> for u8 {
    #[inline(always)]
    fn from(variant: Rtcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcc {
    type Ux = u8;
}
impl crate::IsEnum for Rtcc {}
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Prescaler"]
pub type RtccR = crate::FieldReader<Rtcc>;
impl RtccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rtcc> {
        match self.bits {
            0 => Some(Rtcc::Div1),
            1 => Some(Rtcc::Div2),
            2 => Some(Rtcc::Div4),
            _ => None,
        }
    }
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Rtcc::Div1
    }
    #[doc = "LFECLKRTCC = LFECLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Rtcc::Div2
    }
    #[doc = "LFECLKRTCC = LFECLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Rtcc::Div4
    }
}
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar Prescaler"]
pub type RtccW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcc>;
impl<'a, REG> RtccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFECLKRTCC = LFECLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcc::Div1)
    }
    #[doc = "LFECLKRTCC = LFECLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcc::Div2)
    }
    #[doc = "LFECLKRTCC = LFECLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcc::Div4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&self) -> RtccR {
        RtccR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-Time Counter and Calendar Prescaler"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RtccW<'_, Lfepresc0Spec> {
        RtccW::new(self, 0)
    }
}
#[doc = "Low Frequency E Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfepresc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfepresc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfepresc0Spec;
impl crate::RegisterSpec for Lfepresc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfepresc0::R`](R) reader structure"]
impl crate::Readable for Lfepresc0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfepresc0::W`](W) writer structure"]
impl crate::Writable for Lfepresc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFEPRESC0 to value 0"]
impl crate::Resettable for Lfepresc0Spec {}
