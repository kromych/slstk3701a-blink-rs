#[doc = "Register `CH8_LINK` reader"]
pub type R = crate::R<CH8_LINK_SPEC>;
#[doc = "Register `CH8_LINK` writer"]
pub type W = crate::W<CH8_LINK_SPEC>;
#[doc = "Field `LINKMODE` reader - Link Structure Addressing Mode"]
pub type LINKMODE_R = crate::BitReader;
#[doc = "Field `LINK` reader - Link Next Structure"]
pub type LINK_R = crate::BitReader;
#[doc = "Field `LINK` writer - Link Next Structure"]
pub type LINK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKADDR` reader - Link Structure Address"]
pub type LINKADDR_R = crate::FieldReader<u32>;
#[doc = "Field `LINKADDR` writer - Link Structure Address"]
pub type LINKADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - Link Structure Addressing Mode"]
    #[inline(always)]
    pub fn linkmode(&self) -> LINKMODE_R {
        LINKMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&self) -> LINKADDR_R {
        LINKADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<CH8_LINK_SPEC> {
        LINK_W::new(self, 1)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    #[must_use]
    pub fn linkaddr(&mut self) -> LINKADDR_W<CH8_LINK_SPEC> {
        LINKADDR_W::new(self, 2)
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
#[doc = "Channel Descriptor Link Structure Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch8_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch8_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH8_LINK_SPEC;
impl crate::RegisterSpec for CH8_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch8_link::R`](R) reader structure"]
impl crate::Readable for CH8_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch8_link::W`](W) writer structure"]
impl crate::Writable for CH8_LINK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH8_LINK to value 0"]
impl crate::Resettable for CH8_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
