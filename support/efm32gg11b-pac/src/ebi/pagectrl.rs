#[doc = "Register `PAGECTRL` reader"]
pub type R = crate::R<PagectrlSpec>;
#[doc = "Register `PAGECTRL` writer"]
pub type W = crate::W<PagectrlSpec>;
#[doc = "Page Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pagelen {
    #[doc = "0: 4 members in a page."]
    Member4 = 0,
    #[doc = "1: 8 members in a page."]
    Member8 = 1,
    #[doc = "2: 16 members in a page."]
    Member16 = 2,
    #[doc = "3: 32 members in a page."]
    Member32 = 3,
}
impl From<Pagelen> for u8 {
    #[inline(always)]
    fn from(variant: Pagelen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pagelen {
    type Ux = u8;
}
impl crate::IsEnum for Pagelen {}
#[doc = "Field `PAGELEN` reader - Page Length"]
pub type PagelenR = crate::FieldReader<Pagelen>;
impl PagelenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pagelen {
        match self.bits {
            0 => Pagelen::Member4,
            1 => Pagelen::Member8,
            2 => Pagelen::Member16,
            3 => Pagelen::Member32,
            _ => unreachable!(),
        }
    }
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn is_member4(&self) -> bool {
        *self == Pagelen::Member4
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn is_member8(&self) -> bool {
        *self == Pagelen::Member8
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn is_member16(&self) -> bool {
        *self == Pagelen::Member16
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn is_member32(&self) -> bool {
        *self == Pagelen::Member32
    }
}
#[doc = "Field `PAGELEN` writer - Page Length"]
pub type PagelenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pagelen, crate::Safe>;
impl<'a, REG> PagelenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 members in a page."]
    #[inline(always)]
    pub fn member4(self) -> &'a mut crate::W<REG> {
        self.variant(Pagelen::Member4)
    }
    #[doc = "8 members in a page."]
    #[inline(always)]
    pub fn member8(self) -> &'a mut crate::W<REG> {
        self.variant(Pagelen::Member8)
    }
    #[doc = "16 members in a page."]
    #[inline(always)]
    pub fn member16(self) -> &'a mut crate::W<REG> {
        self.variant(Pagelen::Member16)
    }
    #[doc = "32 members in a page."]
    #[inline(always)]
    pub fn member32(self) -> &'a mut crate::W<REG> {
        self.variant(Pagelen::Member32)
    }
}
#[doc = "Field `INCHIT` reader - Intrapage Hit Only on Incremental Addresses"]
pub type InchitR = crate::BitReader;
#[doc = "Field `INCHIT` writer - Intrapage Hit Only on Incremental Addresses"]
pub type InchitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPA` reader - Page Read Access Time"]
pub type RdpaR = crate::FieldReader;
#[doc = "Field `RDPA` writer - Page Read Access Time"]
pub type RdpaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `KEEPOPEN` reader - Maximum Page Open Time"]
pub type KeepopenR = crate::FieldReader;
#[doc = "Field `KEEPOPEN` writer - Maximum Page Open Time"]
pub type KeepopenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&self) -> PagelenR {
        PagelenR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    pub fn inchit(&self) -> InchitR {
        InchitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&self) -> RdpaR {
        RdpaR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    pub fn keepopen(&self) -> KeepopenR {
        KeepopenR::new(((self.bits >> 20) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Length"]
    #[inline(always)]
    pub fn pagelen(&mut self) -> PagelenW<'_, PagectrlSpec> {
        PagelenW::new(self, 0)
    }
    #[doc = "Bit 4 - Intrapage Hit Only on Incremental Addresses"]
    #[inline(always)]
    pub fn inchit(&mut self) -> InchitW<'_, PagectrlSpec> {
        InchitW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Page Read Access Time"]
    #[inline(always)]
    pub fn rdpa(&mut self) -> RdpaW<'_, PagectrlSpec> {
        RdpaW::new(self, 8)
    }
    #[doc = "Bits 20:26 - Maximum Page Open Time"]
    #[inline(always)]
    pub fn keepopen(&mut self) -> KeepopenW<'_, PagectrlSpec> {
        KeepopenW::new(self, 20)
    }
}
#[doc = "Page Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pagectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PagectrlSpec;
impl crate::RegisterSpec for PagectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pagectrl::R`](R) reader structure"]
impl crate::Readable for PagectrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pagectrl::W`](W) writer structure"]
impl crate::Writable for PagectrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAGECTRL to value 0x0f00"]
impl crate::Resettable for PagectrlSpec {
    const RESET_VALUE: u32 = 0x0f00;
}
