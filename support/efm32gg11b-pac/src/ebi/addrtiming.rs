#[doc = "Register `ADDRTIMING` reader"]
pub type R = crate::R<AddrtimingSpec>;
#[doc = "Register `ADDRTIMING` writer"]
pub type W = crate::W<AddrtimingSpec>;
#[doc = "Field `ADDRSETUP` reader - Address Setup Time"]
pub type AddrsetupR = crate::FieldReader;
#[doc = "Field `ADDRSETUP` writer - Address Setup Time"]
pub type AddrsetupW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDRHOLD` reader - Address Hold Time"]
pub type AddrholdR = crate::FieldReader;
#[doc = "Field `ADDRHOLD` writer - Address Hold Time"]
pub type AddrholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFALE` reader - Half Cycle ALE Strobe Duration Enable"]
pub type HalfaleR = crate::BitReader;
#[doc = "Field `HALFALE` writer - Half Cycle ALE Strobe Duration Enable"]
pub type HalfaleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&self) -> AddrsetupR {
        AddrsetupR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&self) -> AddrholdR {
        AddrholdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&self) -> HalfaleR {
        HalfaleR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address Setup Time"]
    #[inline(always)]
    pub fn addrsetup(&mut self) -> AddrsetupW<'_, AddrtimingSpec> {
        AddrsetupW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Address Hold Time"]
    #[inline(always)]
    pub fn addrhold(&mut self) -> AddrholdW<'_, AddrtimingSpec> {
        AddrholdW::new(self, 8)
    }
    #[doc = "Bit 28 - Half Cycle ALE Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfale(&mut self) -> HalfaleW<'_, AddrtimingSpec> {
        HalfaleW::new(self, 28)
    }
}
#[doc = "Address Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addrtiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrtiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrtimingSpec;
impl crate::RegisterSpec for AddrtimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addrtiming::R`](R) reader structure"]
impl crate::Readable for AddrtimingSpec {}
#[doc = "`write(|w| ..)` method takes [`addrtiming::W`](W) writer structure"]
impl crate::Writable for AddrtimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDRTIMING to value 0x0707"]
impl crate::Resettable for AddrtimingSpec {
    const RESET_VALUE: u32 = 0x0707;
}
