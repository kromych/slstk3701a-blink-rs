#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - CRC Functionality Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - CRC Functionality Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLYSEL` reader - Polynomial Select"]
pub type PolyselR = crate::BitReader;
#[doc = "Field `POLYSEL` writer - Polynomial Select"]
pub type PolyselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTEMODE` reader - Byte Mode Enable"]
pub type BytemodeR = crate::BitReader;
#[doc = "Field `BYTEMODE` writer - Byte Mode Enable"]
pub type BytemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITREVERSE` reader - Byte-level Bit Reverse Enable"]
pub type BitreverseR = crate::BitReader;
#[doc = "Field `BITREVERSE` writer - Byte-level Bit Reverse Enable"]
pub type BitreverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTEREVERSE` reader - Byte Reverse Mode"]
pub type BytereverseR = crate::BitReader;
#[doc = "Field `BYTEREVERSE` writer - Byte Reverse Mode"]
pub type BytereverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOINIT` reader - Auto Init Enable"]
pub type AutoinitR = crate::BitReader;
#[doc = "Field `AUTOINIT` writer - Auto Init Enable"]
pub type AutoinitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&self) -> PolyselR {
        PolyselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&self) -> BytemodeR {
        BytemodeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&self) -> BitreverseR {
        BitreverseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&self) -> BytereverseR {
        BytereverseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&self) -> AutoinitR {
        AutoinitR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&mut self) -> PolyselW<'_, CtrlSpec> {
        PolyselW::new(self, 4)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&mut self) -> BytemodeW<'_, CtrlSpec> {
        BytemodeW::new(self, 8)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&mut self) -> BitreverseW<'_, CtrlSpec> {
        BitreverseW::new(self, 9)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&mut self) -> BytereverseW<'_, CtrlSpec> {
        BytereverseW::new(self, 10)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&mut self) -> AutoinitW<'_, CtrlSpec> {
        AutoinitW::new(self, 13)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
