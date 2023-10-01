#[doc = "Register `EXTILEVEL` reader"]
pub type R = crate::R<EXTILEVEL_SPEC>;
#[doc = "Register `EXTILEVEL` writer"]
pub type W = crate::W<EXTILEVEL_SPEC>;
#[doc = "Field `EM4WU0` reader - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_R = crate::BitReader;
#[doc = "Field `EM4WU0` writer - EM4 Wake Up Level for EM4WU0 Pin"]
pub type EM4WU0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU1` reader - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_R = crate::BitReader;
#[doc = "Field `EM4WU1` writer - EM4 Wake Up Level for EM4WU1 Pin"]
pub type EM4WU1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU2` reader - EM4 Wake Up Level for EM4WU2 Pin"]
pub type EM4WU2_R = crate::BitReader;
#[doc = "Field `EM4WU2` writer - EM4 Wake Up Level for EM4WU2 Pin"]
pub type EM4WU2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU3` reader - EM4 Wake Up Level for EM4WU3 Pin"]
pub type EM4WU3_R = crate::BitReader;
#[doc = "Field `EM4WU3` writer - EM4 Wake Up Level for EM4WU3 Pin"]
pub type EM4WU3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU4` reader - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_R = crate::BitReader;
#[doc = "Field `EM4WU4` writer - EM4 Wake Up Level for EM4WU4 Pin"]
pub type EM4WU4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU5` reader - EM4 Wake Up Level for EM4WU5 Pin"]
pub type EM4WU5_R = crate::BitReader;
#[doc = "Field `EM4WU5` writer - EM4 Wake Up Level for EM4WU5 Pin"]
pub type EM4WU5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU6` reader - EM4 Wake Up Level for EM4WU6 Pin"]
pub type EM4WU6_R = crate::BitReader;
#[doc = "Field `EM4WU6` writer - EM4 Wake Up Level for EM4WU6 Pin"]
pub type EM4WU6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU7` reader - EM4 Wake Up Level for EM4WU7 Pin"]
pub type EM4WU7_R = crate::BitReader;
#[doc = "Field `EM4WU7` writer - EM4 Wake Up Level for EM4WU7 Pin"]
pub type EM4WU7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU8` reader - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_R = crate::BitReader;
#[doc = "Field `EM4WU8` writer - EM4 Wake Up Level for EM4WU8 Pin"]
pub type EM4WU8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM4WU9` reader - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_R = crate::BitReader;
#[doc = "Field `EM4WU9` writer - EM4 Wake Up Level for EM4WU9 Pin"]
pub type EM4WU9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&self) -> EM4WU0_R {
        EM4WU0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&self) -> EM4WU1_R {
        EM4WU1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    pub fn em4wu2(&self) -> EM4WU2_R {
        EM4WU2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    pub fn em4wu3(&self) -> EM4WU3_R {
        EM4WU3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&self) -> EM4WU4_R {
        EM4WU4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    pub fn em4wu5(&self) -> EM4WU5_R {
        EM4WU5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    pub fn em4wu6(&self) -> EM4WU6_R {
        EM4WU6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    pub fn em4wu7(&self) -> EM4WU7_R {
        EM4WU7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&self) -> EM4WU8_R {
        EM4WU8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&self) -> EM4WU9_R {
        EM4WU9_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu0(&mut self) -> EM4WU0_W<EXTILEVEL_SPEC, 16> {
        EM4WU0_W::new(self)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu1(&mut self) -> EM4WU1_W<EXTILEVEL_SPEC, 17> {
        EM4WU1_W::new(self)
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu2(&mut self) -> EM4WU2_W<EXTILEVEL_SPEC, 18> {
        EM4WU2_W::new(self)
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu3(&mut self) -> EM4WU3_W<EXTILEVEL_SPEC, 19> {
        EM4WU3_W::new(self)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu4(&mut self) -> EM4WU4_W<EXTILEVEL_SPEC, 20> {
        EM4WU4_W::new(self)
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu5(&mut self) -> EM4WU5_W<EXTILEVEL_SPEC, 21> {
        EM4WU5_W::new(self)
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu6(&mut self) -> EM4WU6_W<EXTILEVEL_SPEC, 22> {
        EM4WU6_W::new(self)
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu7(&mut self) -> EM4WU7_W<EXTILEVEL_SPEC, 23> {
        EM4WU7_W::new(self)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu8(&mut self) -> EM4WU8_W<EXTILEVEL_SPEC, 24> {
        EM4WU8_W::new(self)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu9(&mut self) -> EM4WU9_W<EXTILEVEL_SPEC, 25> {
        EM4WU9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "External Interrupt Level Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extilevel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extilevel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTILEVEL_SPEC;
impl crate::RegisterSpec for EXTILEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extilevel::R`](R) reader structure"]
impl crate::Readable for EXTILEVEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extilevel::W`](W) writer structure"]
impl crate::Writable for EXTILEVEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTILEVEL to value 0"]
impl crate::Resettable for EXTILEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
