#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `TOUT` reader - WDOG Timeout Interrupt Flag"]
pub type ToutR = crate::BitReader;
#[doc = "Field `WARN` reader - WDOG Warning Timeout Interrupt Flag"]
pub type WarnR = crate::BitReader;
#[doc = "Field `WIN` reader - WDOG Window Interrupt Flag"]
pub type WinR = crate::BitReader;
#[doc = "Field `PEM0` reader - PRS Channel Zero Event Missing Interrupt Flag"]
pub type Pem0R = crate::BitReader;
#[doc = "Field `PEM1` reader - PRS Channel One Event Missing Interrupt Flag"]
pub type Pem1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRS Channel Zero Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&self) -> Pem0R {
        Pem0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRS Channel One Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&self) -> Pem1R {
        Pem1R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Watchdog Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
