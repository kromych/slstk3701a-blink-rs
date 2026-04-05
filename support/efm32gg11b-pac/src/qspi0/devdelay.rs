#[doc = "Register `DEVDELAY` reader"]
pub type R = crate::R<DevdelaySpec>;
#[doc = "Register `DEVDELAY` writer"]
pub type W = crate::W<DevdelaySpec>;
#[doc = "Field `DINIT` reader - Clock Delay for CS"]
pub type DinitR = crate::FieldReader;
#[doc = "Field `DINIT` writer - Clock Delay for CS"]
pub type DinitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DAFTER` reader - Clock Delay for Last Transaction Bit"]
pub type DafterR = crate::FieldReader;
#[doc = "Field `DAFTER` writer - Clock Delay for Last Transaction Bit"]
pub type DafterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBTWN` reader - Clock Delay Between Two Chip Selects"]
pub type DbtwnR = crate::FieldReader;
#[doc = "Field `DBTWN` writer - Clock Delay Between Two Chip Selects"]
pub type DbtwnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DNSS` reader - Clock Delay for Chip Select Deassert"]
pub type DnssR = crate::FieldReader;
#[doc = "Field `DNSS` writer - Clock Delay for Chip Select Deassert"]
pub type DnssW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&self) -> DinitR {
        DinitR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&self) -> DafterR {
        DafterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&self) -> DbtwnR {
        DbtwnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&self) -> DnssR {
        DnssR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Delay for CS"]
    #[inline(always)]
    pub fn dinit(&mut self) -> DinitW<'_, DevdelaySpec> {
        DinitW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock Delay for Last Transaction Bit"]
    #[inline(always)]
    pub fn dafter(&mut self) -> DafterW<'_, DevdelaySpec> {
        DafterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock Delay Between Two Chip Selects"]
    #[inline(always)]
    pub fn dbtwn(&mut self) -> DbtwnW<'_, DevdelaySpec> {
        DbtwnW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock Delay for Chip Select Deassert"]
    #[inline(always)]
    pub fn dnss(&mut self) -> DnssW<'_, DevdelaySpec> {
        DnssW::new(self, 24)
    }
}
#[doc = "Device Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devdelay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devdelay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevdelaySpec;
impl crate::RegisterSpec for DevdelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devdelay::R`](R) reader structure"]
impl crate::Readable for DevdelaySpec {}
#[doc = "`write(|w| ..)` method takes [`devdelay::W`](W) writer structure"]
impl crate::Writable for DevdelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVDELAY to value 0"]
impl crate::Resettable for DevdelaySpec {}
