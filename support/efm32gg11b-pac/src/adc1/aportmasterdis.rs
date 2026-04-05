#[doc = "Register `APORTMASTERDIS` reader"]
pub type R = crate::R<AportmasterdisSpec>;
#[doc = "Register `APORTMASTERDIS` writer"]
pub type W = crate::W<AportmasterdisSpec>;
#[doc = "Field `APORT1XMASTERDIS` reader - APORT1X Master Disable"]
pub type Aport1xmasterdisR = crate::BitReader;
#[doc = "Field `APORT1XMASTERDIS` writer - APORT1X Master Disable"]
pub type Aport1xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT1YMASTERDIS` reader - APORT1Y Master Disable"]
pub type Aport1ymasterdisR = crate::BitReader;
#[doc = "Field `APORT1YMASTERDIS` writer - APORT1Y Master Disable"]
pub type Aport1ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT2XMASTERDIS` reader - APORT2X Master Disable"]
pub type Aport2xmasterdisR = crate::BitReader;
#[doc = "Field `APORT2XMASTERDIS` writer - APORT2X Master Disable"]
pub type Aport2xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT2YMASTERDIS` reader - APORT2Y Master Disable"]
pub type Aport2ymasterdisR = crate::BitReader;
#[doc = "Field `APORT2YMASTERDIS` writer - APORT2Y Master Disable"]
pub type Aport2ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT3XMASTERDIS` reader - APORT3X Master Disable"]
pub type Aport3xmasterdisR = crate::BitReader;
#[doc = "Field `APORT3XMASTERDIS` writer - APORT3X Master Disable"]
pub type Aport3xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT3YMASTERDIS` reader - APORT3Y Master Disable"]
pub type Aport3ymasterdisR = crate::BitReader;
#[doc = "Field `APORT3YMASTERDIS` writer - APORT3Y Master Disable"]
pub type Aport3ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT4XMASTERDIS` reader - APORT4X Master Disable"]
pub type Aport4xmasterdisR = crate::BitReader;
#[doc = "Field `APORT4XMASTERDIS` writer - APORT4X Master Disable"]
pub type Aport4xmasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT4YMASTERDIS` reader - APORT4Y Master Disable"]
pub type Aport4ymasterdisR = crate::BitReader;
#[doc = "Field `APORT4YMASTERDIS` writer - APORT4Y Master Disable"]
pub type Aport4ymasterdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> Aport1xmasterdisR {
        Aport1xmasterdisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> Aport1ymasterdisR {
        Aport1ymasterdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> Aport2xmasterdisR {
        Aport2xmasterdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> Aport2ymasterdisR {
        Aport2ymasterdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> Aport3xmasterdisR {
        Aport3xmasterdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> Aport3ymasterdisR {
        Aport3ymasterdisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> Aport4xmasterdisR {
        Aport4xmasterdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> Aport4ymasterdisR {
        Aport4ymasterdisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&mut self) -> Aport1xmasterdisW<'_, AportmasterdisSpec> {
        Aport1xmasterdisW::new(self, 2)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&mut self) -> Aport1ymasterdisW<'_, AportmasterdisSpec> {
        Aport1ymasterdisW::new(self, 3)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&mut self) -> Aport2xmasterdisW<'_, AportmasterdisSpec> {
        Aport2xmasterdisW::new(self, 4)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&mut self) -> Aport2ymasterdisW<'_, AportmasterdisSpec> {
        Aport2ymasterdisW::new(self, 5)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&mut self) -> Aport3xmasterdisW<'_, AportmasterdisSpec> {
        Aport3xmasterdisW::new(self, 6)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&mut self) -> Aport3ymasterdisW<'_, AportmasterdisSpec> {
        Aport3ymasterdisW::new(self, 7)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&mut self) -> Aport4xmasterdisW<'_, AportmasterdisSpec> {
        Aport4xmasterdisW::new(self, 8)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&mut self) -> Aport4ymasterdisW<'_, AportmasterdisSpec> {
        Aport4ymasterdisW::new(self, 9)
    }
}
#[doc = "APORT Bus Master Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportmasterdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aportmasterdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AportmasterdisSpec;
impl crate::RegisterSpec for AportmasterdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportmasterdis::R`](R) reader structure"]
impl crate::Readable for AportmasterdisSpec {}
#[doc = "`write(|w| ..)` method takes [`aportmasterdis::W`](W) writer structure"]
impl crate::Writable for AportmasterdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APORTMASTERDIS to value 0"]
impl crate::Resettable for AportmasterdisSpec {}
