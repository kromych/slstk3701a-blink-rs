#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Field `FPIOC` reader - FPU invalid operation"]
pub type FpiocR = crate::BitReader;
#[doc = "Field `FPDZC` reader - FPU divide-by-zero exception"]
pub type FpdzcR = crate::BitReader;
#[doc = "Field `FPUFC` reader - FPU underflow exception"]
pub type FpufcR = crate::BitReader;
#[doc = "Field `FPOFC` reader - FPU overflow exception"]
pub type FpofcR = crate::BitReader;
#[doc = "Field `FPIDC` reader - FPU input denormal exception"]
pub type FpidcR = crate::BitReader;
#[doc = "Field `FPIXC` reader - FPU inexact exception"]
pub type FpixcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FPU invalid operation"]
    #[inline(always)]
    pub fn fpioc(&self) -> FpiocR {
        FpiocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FPU divide-by-zero exception"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FpdzcR {
        FpdzcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPU underflow exception"]
    #[inline(always)]
    pub fn fpufc(&self) -> FpufcR {
        FpufcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FPU overflow exception"]
    #[inline(always)]
    pub fn fpofc(&self) -> FpofcR {
        FpofcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FPU input denormal exception"]
    #[inline(always)]
    pub fn fpidc(&self) -> FpidcR {
        FpidcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FPU inexact exception"]
    #[inline(always)]
    pub fn fpixc(&self) -> FpixcR {
        FpixcR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
