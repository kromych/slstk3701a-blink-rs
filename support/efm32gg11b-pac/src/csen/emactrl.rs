#[doc = "Register `EMACTRL` reader"]
pub type R = crate::R<EmactrlSpec>;
#[doc = "Register `EMACTRL` writer"]
pub type W = crate::W<EmactrlSpec>;
#[doc = "EMA Sample Weight\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Emasample {
    #[doc = "0: EMA weight (N) is 1."]
    W1 = 0,
    #[doc = "1: EMA weight (N) is 2."]
    W2 = 1,
    #[doc = "2: EMA weight (N) is 4."]
    W4 = 2,
    #[doc = "3: EMA weight (N) is 8."]
    W8 = 3,
    #[doc = "4: EMA weight (N) is 16."]
    W16 = 4,
    #[doc = "5: EMA weight (N) is 32."]
    W32 = 5,
    #[doc = "6: EMA weight (N) is 64."]
    W64 = 6,
}
impl From<Emasample> for u8 {
    #[inline(always)]
    fn from(variant: Emasample) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Emasample {
    type Ux = u8;
}
impl crate::IsEnum for Emasample {}
#[doc = "Field `EMASAMPLE` reader - EMA Sample Weight"]
pub type EmasampleR = crate::FieldReader<Emasample>;
impl EmasampleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Emasample> {
        match self.bits {
            0 => Some(Emasample::W1),
            1 => Some(Emasample::W2),
            2 => Some(Emasample::W4),
            3 => Some(Emasample::W8),
            4 => Some(Emasample::W16),
            5 => Some(Emasample::W32),
            6 => Some(Emasample::W64),
            _ => None,
        }
    }
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn is_w1(&self) -> bool {
        *self == Emasample::W1
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn is_w2(&self) -> bool {
        *self == Emasample::W2
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn is_w4(&self) -> bool {
        *self == Emasample::W4
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn is_w8(&self) -> bool {
        *self == Emasample::W8
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn is_w16(&self) -> bool {
        *self == Emasample::W16
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn is_w32(&self) -> bool {
        *self == Emasample::W32
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn is_w64(&self) -> bool {
        *self == Emasample::W64
    }
}
#[doc = "Field `EMASAMPLE` writer - EMA Sample Weight"]
pub type EmasampleW<'a, REG> = crate::FieldWriter<'a, REG, 3, Emasample>;
impl<'a, REG> EmasampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EMA weight (N) is 1."]
    #[inline(always)]
    pub fn w1(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W1)
    }
    #[doc = "EMA weight (N) is 2."]
    #[inline(always)]
    pub fn w2(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W2)
    }
    #[doc = "EMA weight (N) is 4."]
    #[inline(always)]
    pub fn w4(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W4)
    }
    #[doc = "EMA weight (N) is 8."]
    #[inline(always)]
    pub fn w8(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W8)
    }
    #[doc = "EMA weight (N) is 16."]
    #[inline(always)]
    pub fn w16(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W16)
    }
    #[doc = "EMA weight (N) is 32."]
    #[inline(always)]
    pub fn w32(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W32)
    }
    #[doc = "EMA weight (N) is 64."]
    #[inline(always)]
    pub fn w64(self) -> &'a mut crate::W<REG> {
        self.variant(Emasample::W64)
    }
}
impl R {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&self) -> EmasampleR {
        EmasampleR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EMA Sample Weight"]
    #[inline(always)]
    pub fn emasample(&mut self) -> EmasampleW<'_, EmactrlSpec> {
        EmasampleW::new(self, 0)
    }
}
#[doc = "Exponential Moving Average Control\n\nYou can [`read`](crate::Reg::read) this register and get [`emactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmactrlSpec;
impl crate::RegisterSpec for EmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emactrl::R`](R) reader structure"]
impl crate::Readable for EmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emactrl::W`](W) writer structure"]
impl crate::Writable for EmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMACTRL to value 0"]
impl crate::Resettable for EmactrlSpec {}
