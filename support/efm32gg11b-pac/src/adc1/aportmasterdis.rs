#[doc = "Register `APORTMASTERDIS` reader"]
pub type R = crate::R<APORTMASTERDIS_SPEC>;
#[doc = "Register `APORTMASTERDIS` writer"]
pub type W = crate::W<APORTMASTERDIS_SPEC>;
#[doc = "Field `APORT1XMASTERDIS` reader - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT1XMASTERDIS` writer - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT1YMASTERDIS` reader - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT1YMASTERDIS` writer - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT2XMASTERDIS` reader - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT2XMASTERDIS` writer - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT2YMASTERDIS` reader - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT2YMASTERDIS` writer - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT3XMASTERDIS` reader - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT3XMASTERDIS` writer - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT3YMASTERDIS` reader - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT3YMASTERDIS` writer - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT4XMASTERDIS` reader - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT4XMASTERDIS` writer - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORT4YMASTERDIS` reader - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORT4YMASTERDIS` writer - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> APORT1XMASTERDIS_R {
        APORT1XMASTERDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> APORT1YMASTERDIS_R {
        APORT1YMASTERDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> APORT2XMASTERDIS_R {
        APORT2XMASTERDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> APORT2YMASTERDIS_R {
        APORT2YMASTERDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> APORT3XMASTERDIS_R {
        APORT3XMASTERDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> APORT3YMASTERDIS_R {
        APORT3YMASTERDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> APORT4XMASTERDIS_R {
        APORT4XMASTERDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> APORT4YMASTERDIS_R {
        APORT4YMASTERDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1xmasterdis(&mut self) -> APORT1XMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT1XMASTERDIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1ymasterdis(&mut self) -> APORT1YMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT1YMASTERDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2xmasterdis(&mut self) -> APORT2XMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT2XMASTERDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2ymasterdis(&mut self) -> APORT2YMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT2YMASTERDIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3xmasterdis(&mut self) -> APORT3XMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT3XMASTERDIS_W::new(self, 6)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3ymasterdis(&mut self) -> APORT3YMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT3YMASTERDIS_W::new(self, 7)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4xmasterdis(&mut self) -> APORT4XMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT4XMASTERDIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4ymasterdis(&mut self) -> APORT4YMASTERDIS_W<APORTMASTERDIS_SPEC> {
        APORT4YMASTERDIS_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "APORT Bus Master Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportmasterdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aportmasterdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTMASTERDIS_SPEC;
impl crate::RegisterSpec for APORTMASTERDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportmasterdis::R`](R) reader structure"]
impl crate::Readable for APORTMASTERDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aportmasterdis::W`](W) writer structure"]
impl crate::Writable for APORTMASTERDIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APORTMASTERDIS to value 0"]
impl crate::Resettable for APORTMASTERDIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
