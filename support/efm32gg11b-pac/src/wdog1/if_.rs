#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `TOUT` reader - WDOG Timeout Interrupt Flag"]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `WARN` reader - WDOG Warning Timeout Interrupt Flag"]
pub type WARN_R = crate::BitReader;
#[doc = "Field `WIN` reader - WDOG Window Interrupt Flag"]
pub type WIN_R = crate::BitReader;
#[doc = "Field `PEM0` reader - PRS Channel Zero Event Missing Interrupt Flag"]
pub type PEM0_R = crate::BitReader;
#[doc = "Field `PEM1` reader - PRS Channel One Event Missing Interrupt Flag"]
pub type PEM1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRS Channel Zero Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRS Channel One Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Watchdog Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
