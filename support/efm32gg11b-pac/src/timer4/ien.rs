#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OfR = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UfR = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` reader - DIRCHG Interrupt Enable"]
pub type DirchgR = crate::BitReader;
#[doc = "Field `DIRCHG` writer - DIRCHG Interrupt Enable"]
pub type DirchgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type Cc0R = crate::BitReader;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type Cc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type Cc1R = crate::BitReader;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type Cc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type Cc2R = crate::BitReader;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type Cc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3` reader - CC3 Interrupt Enable"]
pub type Cc3R = crate::BitReader;
#[doc = "Field `CC3` writer - CC3 Interrupt Enable"]
pub type Cc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` reader - ICBOF0 Interrupt Enable"]
pub type Icbof0R = crate::BitReader;
#[doc = "Field `ICBOF0` writer - ICBOF0 Interrupt Enable"]
pub type Icbof0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` reader - ICBOF1 Interrupt Enable"]
pub type Icbof1R = crate::BitReader;
#[doc = "Field `ICBOF1` writer - ICBOF1 Interrupt Enable"]
pub type Icbof1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` reader - ICBOF2 Interrupt Enable"]
pub type Icbof2R = crate::BitReader;
#[doc = "Field `ICBOF2` writer - ICBOF2 Interrupt Enable"]
pub type Icbof2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF3` reader - ICBOF3 Interrupt Enable"]
pub type Icbof3R = crate::BitReader;
#[doc = "Field `ICBOF3` writer - ICBOF3 Interrupt Enable"]
pub type Icbof3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UfR {
        UfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DirchgR {
        DirchgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&self) -> Cc3R {
        Cc3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&self) -> Icbof0R {
        Icbof0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&self) -> Icbof1R {
        Icbof1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&self) -> Icbof2R {
        Icbof2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&self) -> Icbof3R {
        Icbof3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OfW<'_, IenSpec> {
        OfW::new(self, 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UfW<'_, IenSpec> {
        UfW::new(self, 1)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DirchgW<'_, IenSpec> {
        DirchgW::new(self, 2)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&mut self) -> Cc0W<'_, IenSpec> {
        Cc0W::new(self, 4)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&mut self) -> Cc1W<'_, IenSpec> {
        Cc1W::new(self, 5)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&mut self) -> Cc2W<'_, IenSpec> {
        Cc2W::new(self, 6)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&mut self) -> Cc3W<'_, IenSpec> {
        Cc3W::new(self, 7)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> Icbof0W<'_, IenSpec> {
        Icbof0W::new(self, 8)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> Icbof1W<'_, IenSpec> {
        Icbof1W::new(self, 9)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> Icbof2W<'_, IenSpec> {
        Icbof2W::new(self, 10)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> Icbof3W<'_, IenSpec> {
        Icbof3W::new(self, 11)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
