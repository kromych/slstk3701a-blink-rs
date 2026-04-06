#[doc = "Register `ROUTE` reader"]
pub type R = crate::R<RouteSpec>;
#[doc = "Register `ROUTE` writer"]
pub type W = crate::W<RouteSpec>;
#[doc = "Field `TXPEN` reader - TX Pin Enable"]
pub type TxpenR = crate::BitReader;
#[doc = "Field `TXPEN` writer - TX Pin Enable"]
pub type TxpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RX Pin Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
}
impl From<Rxloc> for u8 {
    #[inline(always)]
    fn from(variant: Rxloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxloc {
    type Ux = u8;
}
impl crate::IsEnum for Rxloc {}
#[doc = "Field `RXLOC` reader - RX Pin Location"]
pub type RxlocR = crate::FieldReader<Rxloc>;
impl RxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxloc> {
        match self.bits {
            0 => Some(Rxloc::Loc0),
            1 => Some(Rxloc::Loc1),
            2 => Some(Rxloc::Loc2),
            3 => Some(Rxloc::Loc3),
            4 => Some(Rxloc::Loc4),
            5 => Some(Rxloc::Loc5),
            6 => Some(Rxloc::Loc6),
            7 => Some(Rxloc::Loc7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Rxloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Rxloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Rxloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Rxloc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Rxloc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Rxloc::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Rxloc::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == Rxloc::Loc7
    }
}
#[doc = "Field `RXLOC` writer - RX Pin Location"]
pub type RxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Rxloc>;
impl<'a, REG> RxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(Rxloc::Loc7)
    }
}
#[doc = "TX Pin Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txloc {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
}
impl From<Txloc> for u8 {
    #[inline(always)]
    fn from(variant: Txloc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txloc {
    type Ux = u8;
}
impl crate::IsEnum for Txloc {}
#[doc = "Field `TXLOC` reader - TX Pin Location"]
pub type TxlocR = crate::FieldReader<Txloc>;
impl TxlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txloc> {
        match self.bits {
            0 => Some(Txloc::Loc0),
            1 => Some(Txloc::Loc1),
            2 => Some(Txloc::Loc2),
            3 => Some(Txloc::Loc3),
            4 => Some(Txloc::Loc4),
            5 => Some(Txloc::Loc5),
            6 => Some(Txloc::Loc6),
            7 => Some(Txloc::Loc7),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == Txloc::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == Txloc::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == Txloc::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == Txloc::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == Txloc::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == Txloc::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == Txloc::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == Txloc::Loc7
    }
}
#[doc = "Field `TXLOC` writer - TX Pin Location"]
pub type TxlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, Txloc>;
impl<'a, REG> TxlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(Txloc::Loc7)
    }
}
impl R {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TxpenR {
        TxpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&self) -> RxlocR {
        RxlocR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&self) -> TxlocR {
        TxlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TxpenW<'_, RouteSpec> {
        TxpenW::new(self, 0)
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&mut self) -> RxlocW<'_, RouteSpec> {
        RxlocW::new(self, 2)
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&mut self) -> TxlocW<'_, RouteSpec> {
        TxlocW::new(self, 8)
    }
}
#[doc = "I/O Routing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouteSpec;
impl crate::RegisterSpec for RouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`route::R`](R) reader structure"]
impl crate::Readable for RouteSpec {}
#[doc = "`write(|w| ..)` method takes [`route::W`](W) writer structure"]
impl crate::Writable for RouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for RouteSpec {}
