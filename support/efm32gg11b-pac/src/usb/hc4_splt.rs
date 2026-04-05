#[doc = "Register `HC4_SPLT` reader"]
pub type R = crate::R<Hc4SpltSpec>;
#[doc = "Register `HC4_SPLT` writer"]
pub type W = crate::W<Hc4SpltSpec>;
#[doc = "Field `PRTADDR` reader - Port Address"]
pub type PrtaddrR = crate::FieldReader;
#[doc = "Field `PRTADDR` writer - Port Address"]
pub type PrtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HUBADDR` reader - Hub Address"]
pub type HubaddrR = crate::FieldReader;
#[doc = "Field `HUBADDR` writer - Hub Address"]
pub type HubaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `XACTPOS` reader - Transaction Position"]
pub type XactposR = crate::FieldReader;
#[doc = "Field `XACTPOS` writer - Transaction Position"]
pub type XactposW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMPSPLT` reader - Do Complete Split"]
pub type CompspltR = crate::BitReader;
#[doc = "Field `COMPSPLT` writer - Do Complete Split"]
pub type CompspltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLTENA` reader - Split Enable"]
pub type SpltenaR = crate::BitReader;
#[doc = "Field `SPLTENA` writer - Split Enable"]
pub type SpltenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&self) -> PrtaddrR {
        PrtaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&self) -> HubaddrR {
        HubaddrR::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&self) -> XactposR {
        XactposR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&self) -> CompspltR {
        CompspltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&self) -> SpltenaR {
        SpltenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Port Address"]
    #[inline(always)]
    pub fn prtaddr(&mut self) -> PrtaddrW<'_, Hc4SpltSpec> {
        PrtaddrW::new(self, 0)
    }
    #[doc = "Bits 7:13 - Hub Address"]
    #[inline(always)]
    pub fn hubaddr(&mut self) -> HubaddrW<'_, Hc4SpltSpec> {
        HubaddrW::new(self, 7)
    }
    #[doc = "Bits 14:15 - Transaction Position"]
    #[inline(always)]
    pub fn xactpos(&mut self) -> XactposW<'_, Hc4SpltSpec> {
        XactposW::new(self, 14)
    }
    #[doc = "Bit 16 - Do Complete Split"]
    #[inline(always)]
    pub fn compsplt(&mut self) -> CompspltW<'_, Hc4SpltSpec> {
        CompspltW::new(self, 16)
    }
    #[doc = "Bit 31 - Split Enable"]
    #[inline(always)]
    pub fn spltena(&mut self) -> SpltenaW<'_, Hc4SpltSpec> {
        SpltenaW::new(self, 31)
    }
}
#[doc = "Host Channel x Split Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc4_splt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc4_splt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc4SpltSpec;
impl crate::RegisterSpec for Hc4SpltSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc4_splt::R`](R) reader structure"]
impl crate::Readable for Hc4SpltSpec {}
#[doc = "`write(|w| ..)` method takes [`hc4_splt::W`](W) writer structure"]
impl crate::Writable for Hc4SpltSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC4_SPLT to value 0"]
impl crate::Resettable for Hc4SpltSpec {}
