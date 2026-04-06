#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<RoutepenSpec>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<RoutepenSpec>;
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type Cc0penR = crate::BitReader;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type Cc0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type Cc1penR = crate::BitReader;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type Cc1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type Cc2penR = crate::BitReader;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type Cc2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3PEN` reader - CC Channel 3 Pin Enable"]
pub type Cc3penR = crate::BitReader;
#[doc = "Field `CC3PEN` writer - CC Channel 3 Pin Enable"]
pub type Cc3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti0penR = crate::BitReader;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti1penR = crate::BitReader;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti2penR = crate::BitReader;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type Cdti2penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> Cc0penR {
        Cc0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> Cc1penR {
        Cc1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> Cc2penR {
        Cc2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&self) -> Cc3penR {
        Cc3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> Cdti0penR {
        Cdti0penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> Cdti1penR {
        Cdti1penR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> Cdti2penR {
        Cdti2penR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&mut self) -> Cc0penW<'_, RoutepenSpec> {
        Cc0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&mut self) -> Cc1penW<'_, RoutepenSpec> {
        Cc1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&mut self) -> Cc2penW<'_, RoutepenSpec> {
        Cc2penW::new(self, 2)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&mut self) -> Cc3penW<'_, RoutepenSpec> {
        Cc3penW::new(self, 3)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&mut self) -> Cdti0penW<'_, RoutepenSpec> {
        Cdti0penW::new(self, 8)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&mut self) -> Cdti1penW<'_, RoutepenSpec> {
        Cdti1penW::new(self, 9)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&mut self) -> Cdti2penW<'_, RoutepenSpec> {
        Cdti2penW::new(self, 10)
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RoutepenSpec;
impl crate::RegisterSpec for RoutepenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for RoutepenSpec {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for RoutepenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for RoutepenSpec {}
