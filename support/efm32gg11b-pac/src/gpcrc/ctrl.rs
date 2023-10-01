#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - CRC Functionality Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - CRC Functionality Enable"]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `POLYSEL` reader - Polynomial Select"]
pub type POLYSEL_R = crate::BitReader;
#[doc = "Field `POLYSEL` writer - Polynomial Select"]
pub type POLYSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYTEMODE` reader - Byte Mode Enable"]
pub type BYTEMODE_R = crate::BitReader;
#[doc = "Field `BYTEMODE` writer - Byte Mode Enable"]
pub type BYTEMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BITREVERSE` reader - Byte-level Bit Reverse Enable"]
pub type BITREVERSE_R = crate::BitReader;
#[doc = "Field `BITREVERSE` writer - Byte-level Bit Reverse Enable"]
pub type BITREVERSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYTEREVERSE` reader - Byte Reverse Mode"]
pub type BYTEREVERSE_R = crate::BitReader;
#[doc = "Field `BYTEREVERSE` writer - Byte Reverse Mode"]
pub type BYTEREVERSE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUTOINIT` reader - Auto Init Enable"]
pub type AUTOINIT_R = crate::BitReader;
#[doc = "Field `AUTOINIT` writer - Auto Init Enable"]
pub type AUTOINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    pub fn polysel(&self) -> POLYSEL_R {
        POLYSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    pub fn bytemode(&self) -> BYTEMODE_R {
        BYTEMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    pub fn bitreverse(&self) -> BITREVERSE_R {
        BITREVERSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    pub fn bytereverse(&self) -> BYTEREVERSE_R {
        BYTEREVERSE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    pub fn autoinit(&self) -> AUTOINIT_R {
        AUTOINIT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Functionality Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC, 0> {
        EN_W::new(self)
    }
    #[doc = "Bit 4 - Polynomial Select"]
    #[inline(always)]
    #[must_use]
    pub fn polysel(&mut self) -> POLYSEL_W<CTRL_SPEC, 4> {
        POLYSEL_W::new(self)
    }
    #[doc = "Bit 8 - Byte Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bytemode(&mut self) -> BYTEMODE_W<CTRL_SPEC, 8> {
        BYTEMODE_W::new(self)
    }
    #[doc = "Bit 9 - Byte-level Bit Reverse Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bitreverse(&mut self) -> BITREVERSE_W<CTRL_SPEC, 9> {
        BITREVERSE_W::new(self)
    }
    #[doc = "Bit 10 - Byte Reverse Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bytereverse(&mut self) -> BYTEREVERSE_W<CTRL_SPEC, 10> {
        BYTEREVERSE_W::new(self)
    }
    #[doc = "Bit 13 - Auto Init Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autoinit(&mut self) -> AUTOINIT_W<CTRL_SPEC, 13> {
        AUTOINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
