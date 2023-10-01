#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPEN_SPEC>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPEN_SPEC>;
#[doc = "Field `SCLKPEN` reader - SCLK Pin Enable"]
pub type SCLKPEN_R = crate::BitReader;
#[doc = "Field `SCLKPEN` writer - SCLK Pin Enable"]
pub type SCLKPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS0PEN` reader - CS0 Pin Enable"]
pub type CS0PEN_R = crate::BitReader;
#[doc = "Field `CS0PEN` writer - CS0 Pin Enable"]
pub type CS0PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CS1PEN` reader - CS1 Pin Enable"]
pub type CS1PEN_R = crate::BitReader;
#[doc = "Field `CS1PEN` writer - CS1 Pin Enable"]
pub type CS1PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ0PEN` reader - DQ0 Pin Enable"]
pub type DQ0PEN_R = crate::BitReader;
#[doc = "Field `DQ0PEN` writer - DQ0 Pin Enable"]
pub type DQ0PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ1PEN` reader - DQ1 Pin Enable"]
pub type DQ1PEN_R = crate::BitReader;
#[doc = "Field `DQ1PEN` writer - DQ1 Pin Enable"]
pub type DQ1PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ2PEN` reader - DQ2 Pin Enable"]
pub type DQ2PEN_R = crate::BitReader;
#[doc = "Field `DQ2PEN` writer - DQ2 Pin Enable"]
pub type DQ2PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ3PEN` reader - DQ3 Pin Enable"]
pub type DQ3PEN_R = crate::BitReader;
#[doc = "Field `DQ3PEN` writer - DQ3 Pin Enable"]
pub type DQ3PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ4PEN` reader - DQ4 Pin Enable"]
pub type DQ4PEN_R = crate::BitReader;
#[doc = "Field `DQ4PEN` writer - DQ4 Pin Enable"]
pub type DQ4PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ5PEN` reader - DQ5 Pin Enable"]
pub type DQ5PEN_R = crate::BitReader;
#[doc = "Field `DQ5PEN` writer - DQ5 Pin Enable"]
pub type DQ5PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ6PEN` reader - DQ6 Pin Enable"]
pub type DQ6PEN_R = crate::BitReader;
#[doc = "Field `DQ6PEN` writer - DQ6 Pin Enable"]
pub type DQ6PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQ7PEN` reader - DQ7 Pin Enable"]
pub type DQ7PEN_R = crate::BitReader;
#[doc = "Field `DQ7PEN` writer - DQ7 Pin Enable"]
pub type DQ7PEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DQSPEN` reader - DQS Pin Enable"]
pub type DQSPEN_R = crate::BitReader;
#[doc = "Field `DQSPEN` writer - DQS Pin Enable"]
pub type DQSPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCLKINPEN` reader - SCLKIN Pin Enable"]
pub type SCLKINPEN_R = crate::BitReader;
#[doc = "Field `SCLKINPEN` writer - SCLKIN Pin Enable"]
pub type SCLKINPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    pub fn sclkpen(&self) -> SCLKPEN_R {
        SCLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    pub fn cs0pen(&self) -> CS0PEN_R {
        CS0PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    pub fn cs1pen(&self) -> CS1PEN_R {
        CS1PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    pub fn dq0pen(&self) -> DQ0PEN_R {
        DQ0PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    pub fn dq1pen(&self) -> DQ1PEN_R {
        DQ1PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    pub fn dq2pen(&self) -> DQ2PEN_R {
        DQ2PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    pub fn dq3pen(&self) -> DQ3PEN_R {
        DQ3PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    pub fn dq4pen(&self) -> DQ4PEN_R {
        DQ4PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    pub fn dq5pen(&self) -> DQ5PEN_R {
        DQ5PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    pub fn dq6pen(&self) -> DQ6PEN_R {
        DQ6PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    pub fn dq7pen(&self) -> DQ7PEN_R {
        DQ7PEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    pub fn dqspen(&self) -> DQSPEN_R {
        DQSPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    pub fn sclkinpen(&self) -> SCLKINPEN_R {
        SCLKINPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCLK Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclkpen(&mut self) -> SCLKPEN_W<ROUTEPEN_SPEC, 0> {
        SCLKPEN_W::new(self)
    }
    #[doc = "Bit 1 - CS0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs0pen(&mut self) -> CS0PEN_W<ROUTEPEN_SPEC, 1> {
        CS0PEN_W::new(self)
    }
    #[doc = "Bit 2 - CS1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cs1pen(&mut self) -> CS1PEN_W<ROUTEPEN_SPEC, 2> {
        CS1PEN_W::new(self)
    }
    #[doc = "Bit 5 - DQ0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq0pen(&mut self) -> DQ0PEN_W<ROUTEPEN_SPEC, 5> {
        DQ0PEN_W::new(self)
    }
    #[doc = "Bit 6 - DQ1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq1pen(&mut self) -> DQ1PEN_W<ROUTEPEN_SPEC, 6> {
        DQ1PEN_W::new(self)
    }
    #[doc = "Bit 7 - DQ2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq2pen(&mut self) -> DQ2PEN_W<ROUTEPEN_SPEC, 7> {
        DQ2PEN_W::new(self)
    }
    #[doc = "Bit 8 - DQ3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq3pen(&mut self) -> DQ3PEN_W<ROUTEPEN_SPEC, 8> {
        DQ3PEN_W::new(self)
    }
    #[doc = "Bit 9 - DQ4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq4pen(&mut self) -> DQ4PEN_W<ROUTEPEN_SPEC, 9> {
        DQ4PEN_W::new(self)
    }
    #[doc = "Bit 10 - DQ5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq5pen(&mut self) -> DQ5PEN_W<ROUTEPEN_SPEC, 10> {
        DQ5PEN_W::new(self)
    }
    #[doc = "Bit 11 - DQ6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq6pen(&mut self) -> DQ6PEN_W<ROUTEPEN_SPEC, 11> {
        DQ6PEN_W::new(self)
    }
    #[doc = "Bit 12 - DQ7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dq7pen(&mut self) -> DQ7PEN_W<ROUTEPEN_SPEC, 12> {
        DQ7PEN_W::new(self)
    }
    #[doc = "Bit 13 - DQS Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dqspen(&mut self) -> DQSPEN_W<ROUTEPEN_SPEC, 13> {
        DQSPEN_W::new(self)
    }
    #[doc = "Bit 14 - SCLKIN Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sclkinpen(&mut self) -> SCLKINPEN_W<ROUTEPEN_SPEC, 14> {
        SCLKINPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
